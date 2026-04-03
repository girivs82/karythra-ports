#![no_std]
#![no_main]

use karythra::gpu;
use karythra::ipc;
use karythra::io;
use karythra::wm;

#[karythra::main]
async fn main() {
    io::print("gputest: starting\n");

    let mut gpu_id = 0u32;
    for _ in 0..30u32 {
        if let Some(id) = ipc::lookup("gpu") {
            gpu_id = id;
            break;
        }
        karythra::time::sleep(500).await;
    }
    if gpu_id == 0 {
        io::print("gputest: gpu service not found\n");
        return;
    }
    io::print("gputest: found gpu service\n");

    let window = match wm::Window::create(400, 300, "gputest").await {
        Ok(w) => w,
        Err(_) => { io::print("gputest: window create failed\n"); return; }
    };
    let (cw, ch) = window.content_size();
    io::print("gputest: window created\n");

    // red material
    let mut red_blob = [0u8; 128];
    red_blob[0] = 0x05;
    red_blob[2] = 0xFF;
    red_blob[5] = 0xFF;
    let red_hash = match gpu::upload(gpu_id, &red_blob).await {
        Ok(h) => h,
        Err(_) => { io::print("gputest: upload mat failed\n"); return; }
    };

    // identity transform
    let id_xform = gpu::build_transform(0.0, 0.0, 0.0);
    let id_hash = match gpu::upload(gpu_id, &id_xform).await {
        Ok(h) => h,
        Err(_) => { io::print("gputest: upload xform failed\n"); return; }
    };

    // triangle base vertices
    let cx = cw as f32 / 2.0;
    let cy = ch as f32 / 2.0;
    let r = if cw < ch { cw } else { ch } as f32 * 0.4;
    let base_verts: [f32; 9] = [
        cx,      -(cy - r), 0.0,
        cx - r,  -(cy + r), 0.0,
        cx + r,  -(cy + r), 0.0,
    ];

    // text node (uploaded once, reused every frame)
    let font_hash = gpu::system_font_hash();
    let text_group_h = if let Some(ref fh) = font_hash {
        io::print("gputest: system font found\n");
        let text_blob = gpu::build_text_node(fh, "Hello Karythra", 20.0, 0xFFFFFFFF);
        let text_h = match gpu::upload(gpu_id, &text_blob).await {
            Ok(h) => h,
            Err(_) => { io::print("gputest: upload text failed\n"); [0u8; 32] }
        };
        if text_h != [0u8; 32] {
            let text_xform = gpu::build_transform(10.0, -20.0, 0.0);
            let text_xform_h = match gpu::upload(gpu_id, &text_xform).await {
                Ok(h) => h, Err(_) => id_hash
            };
            let text_list = gpu::build_node_list(&[text_h]);
            let text_list_h = match gpu::upload(gpu_id, &text_list).await {
                Ok(h) => h, Err(_) => [0u8; 32]
            };
            let null_hash = [0u8; 32];
            let tg = gpu::build_scene_node_with_children(0x01, &text_xform_h, &null_hash, &text_list_h);
            match gpu::upload(gpu_id, &tg).await {
                Ok(h) => Some(h), Err(_) => None
            }
        } else { None }
    } else {
        io::print("gputest: no system font\n");
        None
    };

    io::print("gputest: scene built\n");

    // animate: rebuild triangle each frame, include text in content group
    let mut angle: f32 = 0.0;
    let mut frames: u32 = 0;
    let mut last_report = karythra::time::uptime_ms();
    loop {
        let cos = gpu::cos_approx(angle);
        let sin = gpu::sin_approx(angle);

        let mut rot_verts = [0.0f32; 9];
        for i in 0..3 {
            let dx = base_verts[i * 3] - cx;
            let dy = base_verts[i * 3 + 1] - (-cy);
            rot_verts[i * 3]     = cx + dx * cos - dy * sin;
            rot_verts[i * 3 + 1] = -cy + dx * sin + dy * cos;
            rot_verts[i * 3 + 2] = 0.0;
        }

        let mut vb = [0u8; 36];
        for i in 0..9 {
            vb[i * 4..(i + 1) * 4].copy_from_slice(&rot_verts[i].to_le_bytes());
        }
        let vh = match gpu::upload(gpu_id, &vb).await { Ok(h) => h, Err(_) => { angle += 0.03; continue } };

        let mut mb = [0u8; 128];
        mb[0] = 0x08;
        mb[2..6].copy_from_slice(&3u32.to_le_bytes());
        mb[10..12].copy_from_slice(&12u16.to_le_bytes());
        mb[32..64].copy_from_slice(&vh);
        let mh = match gpu::upload(gpu_id, &mb).await { Ok(h) => h, Err(_) => { angle += 0.03; continue } };

        let rb = gpu::build_renderable(&mh, &red_hash);
        let rh = match gpu::upload(gpu_id, &rb).await { Ok(h) => h, Err(_) => { angle += 0.03; continue } };
        let nb = gpu::build_scene_node(0x01, &id_hash, &rh);
        let tri_h = match gpu::upload(gpu_id, &nb).await { Ok(h) => h, Err(_) => { angle += 0.03; continue } };

        // build content: triangle + text group (if available)
        let content = if let Some(ref tgh) = text_group_h {
            let null_hash = [0u8; 32];
            let list = gpu::build_node_list(&[tri_h, *tgh]);
            let list_h = match gpu::upload(gpu_id, &list).await { Ok(h) => h, Err(_) => tri_h };
            let group = gpu::build_scene_node_with_children(0x01, &id_hash, &null_hash, &list_h);
            match gpu::upload(gpu_id, &group).await { Ok(h) => h, Err(_) => tri_h }
        } else {
            tri_h
        };

        let _ = window.set_content(&content).await;

        angle += 0.03;
        frames += 1;
        let now = karythra::time::uptime_ms();
        let elapsed = now - last_report;
        if elapsed >= 5000 {
            io::print("gputest: ");
            io::print_dec(frames as u64);
            io::print(" frames in ");
            io::print_dec(elapsed / 1000);
            io::print("s = ");
            if elapsed > 0 {
                io::print_dec((frames as u64 * 1000) / elapsed);
            }
            io::print(" fps\n");
            frames = 0;
            last_report = now;
        }

        karythra::time::sleep(16).await;
    }
}
