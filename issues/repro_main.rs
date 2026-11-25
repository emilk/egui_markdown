use egui_kittest::Harness;

const CASES: &[(&str, &str)] = &[
  ("list_code", "1. An item with a code block:\n\n   ```\n   let x = 42;\n   ```\n2. A second item\n"),
  ("quote_list_code", "> - A list in a quote\n>\n>   ```\n>   let x = 42;\n>   ```\n"),
  ("table_after_paragraph", "A paragraph\n| a | b |\n|---|---|\n| 1 | 2 |\n"),
  ("table_last", "Text before\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n"),
  ("table_then_text", "| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n\nText after\n"),
  ("two_tables", "| a | b |\n|---|---|\n| 1 | 2 |\n\nText between\n\n| c | d |\n|---|---|\n| 3 | 4 |\n"),
  ("inline_code", "Inline `code` and `fn main() {}`, and a block:\n\n```\nfn main() {}\n```\n"),
];

fn main() {
  for (name, markdown) in CASES {
    let markdown = markdown.to_string();
    let mut harness = Harness::builder().with_size(egui::vec2(320.0, 2000.0)).with_pixels_per_point(2.0).build_ui(move |ui| {
      egui::Frame::central_panel(ui.style()).show(ui, |ui| {
        ui.set_width(300.0);
        md::MarkdownLabel::new(egui::Id::unique("md"), &markdown).show(ui);
      });
    });
    harness.run();
    harness.fit_contents();
    harness.render().expect("render").save(format!("{name}.png")).ok();
  }
}
