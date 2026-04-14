use eframe::egui::{self, Color32};

#[derive(Debug, PartialEq)]
enum Risco{
    Baixo, Medio, Alto
}

#[derive(Debug, PartialEq)]
struct Ativo{
    ticker: String,
    nome: String,
    preco_atual: f64,
    risco: Risco,
    selecionado: bool
}

#[derive(Debug, PartialEq)]
struct OrdemExecutada{
    identificador: usize,
    ticker: String,
    quantidade: f64,
    preco_pago: f64,
}

#[derive(Debug, PartialEq)]
struct MyApp{

    ativos_disponiveis: Vec<Ativo>,
    ativo_selecionado: Vec<usize>,
    ordem_executada: Vec<OrdemExecutada>,
    quantidade_compra: String,
    saldo_conta: f64

}

impl Default for MyApp{
    fn default() -> Self {
      Self { ativos_disponiveis: 
            vec![Ativo {
            ticker: "ITUB4".to_string(),
            nome: "Itaú Unibanco".to_string(),
            preco_atual: 34.50,
            risco: Risco::Baixo,
            selecionado: false
        },
        Ativo {
            ticker: "PETR4".to_string(),
            nome: "Petrobras PN".to_string(),
            preco_atual: 41.20,
            risco: Risco::Medio,
            selecionado: false
        },
        Ativo {
            ticker: "MGLU3".to_string(),
            nome: "Magazine Luiza".to_string(),
            preco_atual: 1.85,
            risco: Risco::Alto,
            selecionado: false
        }], 
        ativo_selecionado: Vec::new(), quantidade_compra: String::new(), saldo_conta: 10000.0, ordem_executada: Vec::new() }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::SidePanel::left("menu_lateral").resizable(true).min_width(150.0).show(ctx, |ui|{

            ui.heading("Mercado");
            ui.separator();

            if self.ativo_selecionado.is_empty(){

                for i in &mut self.ativos_disponiveis.iter_mut().enumerate() {

                    if ui.selectable_label(i.1.selecionado, &i.1.nome).clicked() {
                        if !i.1.selecionado {
                            i.1.selecionado = true;
                            self.ativo_selecionado.push(i.0);
                        }
                        else{
                            i.1.selecionado = false;
                            self.ativo_selecionado.retain(|f| *f != i.0);
                        }
                    }

                }

            }
            else{
                for i in &self.ativos_disponiveis{
                    ui.label(&i.nome);
                }
            }

        });

        egui::TopBottomPanel::bottom("historico").resizable(true).max_height(200.0).show(ctx, |ui|{
            ui.heading("Histórico");
            ui.separator();
            ui.add_space(10.0);

            if !self.ativo_selecionado.is_empty(){

                let item_selecionado = self.ativo_selecionado.get(0);
                let historico = self.ordem_executada
                .iter()
                .filter(|f| f.identificador == *item_selecionado.unwrap())
                .collect::<Vec<&OrdemExecutada>>();

                if !historico.is_empty(){

                    ui.horizontal_centered(|side_ui|{

                        for i in historico{
                            side_ui.label(&i.ticker);
                            side_ui.label(format!("Quantidade {}", i.quantidade));
                            side_ui.label(format!("R$ {:.2}", i.preco_pago));
                            side_ui.label(format!("Valor total R$ {:.2}", (i.quantidade * i.preco_pago)));
                        }

                    });

                }

            }

        });

        egui::CentralPanel::default().show(ctx, |ui|{
            egui::ScrollArea::both().show(ui, |scr|{
                if self.ativo_selecionado.is_empty() {

                    scr.columns(1, |col|{
                        col[0].horizontal_centered(|ui_center|{
                            ui_center
                            .label(egui::RichText::new("Selecione um ativo no menu lateral para visualizar detalhes")
                            .color(Color32::DARK_RED)
                            .size(22.0)
                            .strong());
                        })
                    });
                }
                else{

                    let mut qtde_calc:f64 = 1.00;
                    let mut liberar_ativo = false;
                    let item = self.ativo_selecionado.get(0);
                    
                    scr.horizontal(|scr_hor|{
                        scr_hor.label("Quantidade");
                        scr_hor.text_edit_singleline(&mut self.quantidade_compra);
                    });

                    if let Err(_) = self.quantidade_compra.trim().parse::<f64>() {
                        scr.label(egui::RichText::new("Quantidade Invalida").size(30.0).strong().color(Color32::RED));
                    }
                    else{
                        qtde_calc = self.quantidade_compra.trim().parse::<f64>().unwrap();
                        scr.columns(1, |col|{

                        let ativo_diposnivel = self.ativos_disponiveis.get_mut(*item.unwrap()).unwrap();
                        col[0].horizontal_centered(|col_internal|{

                            col_internal.label(egui::RichText::new(&ativo_diposnivel.ticker).color(Color32::DARK_RED).size(22.0));
                            col_internal.label(egui::RichText::new(&ativo_diposnivel.nome).color(Color32::DARK_RED).size(22.0));
                            col_internal.label(egui::RichText::new(format!("R$ {}", ativo_diposnivel.preco_atual.to_string())).color(Color32::DARK_RED).size(22.0));
                            col_internal.label(format!("Total: R$ {}", (qtde_calc * ativo_diposnivel.preco_atual)));

                            if col_internal.button("Comprar").clicked(){

                                self.saldo_conta -= qtde_calc * ativo_diposnivel.preco_atual;
                                self.ordem_executada.push(OrdemExecutada { 
                                    identificador: *item.unwrap(),
                                    ticker: ativo_diposnivel.ticker.clone(), 
                                    quantidade: qtde_calc, 
                                    preco_pago: ativo_diposnivel.preco_atual });
                                liberar_ativo = true;

                            }

                            });
                        });
                    }
                    if liberar_ativo {
                        self.ativo_selecionado.pop();
                    }
                }     
            })
        });
    }
}

fn main() -> eframe::Result<()>{
    
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0,600.0]), //w - h
        ..Default::default()
    };

    eframe::run_native("Painel cotação", options, Box::new(|_cc| Box::<MyApp>::default()))

}