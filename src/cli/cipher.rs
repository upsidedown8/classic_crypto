use structopt::{StructOpt, clap::arg_enum};

arg_enum! {
    #[derive(StructOpt, Debug)]
    #[structopt(rename_all = "snake")]
    pub enum Cipher {
        Affine,
        Caesar,
    }
}
