use eframe::egui;
use im_native_dialog::ImNativeFileDialog;
use std::path::PathBuf;

mod lfsr;

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn convert_file() {
    let mut lfsr = lfsr::LFSR::new(0b11010010110101, (1 << (31 - 1)) + (1 << (2 - 1)));
    let in_f = File::open("Render_1.enc").unwrap();
    let in_f = BufReader::new(in_f);
    let out_f = File::create("Render_1_dec.mp4").unwrap();
    let mut out_f = BufWriter::new(out_f);

    for byte in in_f.bytes() {
        let byte = byte.unwrap();
        out_f.write(&[byte ^ lfsr.get_byte()]).unwrap();
    }
}

struct MyApp {
    input_file: PathBuf,
    output_file: PathBuf,
    output_file_dialog: ImNativeFileDialog<Option<PathBuf>>,
    input_file_dialog: ImNativeFileDialog<Option<PathBuf>>,
    initial_state: String,

    key_text: String,
    input_text: String,
    output_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            initial_state: "1000000000000000000000000000000".to_string(),
            input_file: Default::default(),
            output_file: Default::default(),
            output_file_dialog: Default::default(),
            input_file_dialog: Default::default(),

            key_text: Default::default(),
            input_text: Default::default(),
            output_text: Default::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.3);

        if let Some(Ok(Some(path))) = self.input_file_dialog.check() {
            self.input_file = path;
        }

        if let Some(Ok(Some(path))) = self.output_file_dialog.check() {
            self.output_file = path;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Min).with_cross_justify(true),
                |ui| {
                    ui.group(|ui| {
                        ui.label(self.input_file.to_str().unwrap());
                        if ui.button("Входной файл...").clicked() {
                            self.input_file_dialog
                                .open_single_file(None)
                                .expect("Unable to open file_path dialog");
                        };
                    });

                    ui.group(|ui| {
                        ui.label(self.output_file.to_str().unwrap());
                        if ui.button("Выходной файл...").clicked() {
                            self.output_file_dialog
                                .show_save_single_file(None)
                                .expect("Unable to open file_path dialog");
                        };
                    });

                    if ui.button("Конвертировать").clicked() {
                        let state = self
                            .initial_state
                            .chars()
                            .filter(|c| *c == '0' || *c == '1')
                            .take(31)
                            .fold(0u32, |acc, c| acc << 1 | (c == '1') as u32);

                        let taps = (1 << (31 - 1)) + (1 << (2 - 1)) + 1; // 31 bit taps

                        let mut lfsr = lfsr::LFSR::new(state, taps);
                        let in_f = BufReader::new(File::open(&self.input_file).unwrap());
                        let mut out_f = BufWriter::new(File::create(&self.output_file).unwrap());

                        self.input_text.clear();
                        self.output_text.clear();
                        self.key_text.clear();

                        for byte in in_f.bytes() {
                            let byte = byte.unwrap();
                            let key_byte = lfsr.get_byte();
                            let output_byte = byte ^ key_byte;

                            if self.input_text.len() < 9 * 5000 {
                                self.input_text.push_str(&format!("{:08b}|", byte));
                                self.output_text.push_str(&format!("{:08b}|", output_byte));
                                self.key_text.push_str(&format!("{:08b}|", key_byte));
                            }

                            out_f.write(&[output_byte]).unwrap();
                        }
                    };
                },
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Начальное состояние регистра:");
                ui.text_edit_singleline(&mut self.initial_state);
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.columns(3, |column| {
                    column[0].label("Ключ");
                    column[0].text_edit_multiline(&mut self.key_text);
                    column[1].label("Входной файл");
                    column[1].text_edit_multiline(&mut self.input_text);
                    column[2].label("Выходной файл");
                    column[2].text_edit_multiline(&mut self.output_text);
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
