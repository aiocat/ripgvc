// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub fn draw_file(text: &str, color: String, round: bool) -> String {
    let width = (12 + (text.len() * 8)) + 3;
    let round_corner = if round { "rx=\"5\" ry=\"5\"" } else { "" };

    format!(
        r#"<!--
Copyright (c) 2022 aiocat

This software is released under the MIT License.
https://opensource.org/licenses/MIT
-->
    
<svg height="20" xmlns="http://www.w3.org/2000/svg">
    <rect width="80" height="20" style="fill:#2a2a2a;" {3} />
    <text x="6" font-size="11" font-family="Verdana,monospace,sans-serif" y="15" fill="black">View Count</text>
    <text x="5" font-size="11" font-family="Verdana,monospace,sans-serif" y="14" fill="white">View Count</text>
    
    <g>
        <rect x="70" width="{0}" height="20" style="fill:{2};" {3} />
        <svg x="70" width="{0}" height="20">
            <text x="51%" font-size="11" text-anchor="middle" y="15" font-family="Verdana,monospace,sans-serif" fill="black">{1}</text>
            <text x="50%" font-size="11" text-anchor="middle" y="14" font-family="Verdana,monospace,sans-serif" fill="white">{1}</text>
        </svg>
    </g>

    <rect x="70" width="3" height="20" style="fill:#2a2a2a;" />
</svg>
"#,
        width, text, color, round_corner
    )
}
