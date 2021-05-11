use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(StructOpt, Debug)]
    #[structopt(rename_all = "snake")]
    pub enum Cipher {
        Affine,
        Caesar,
    }
}
