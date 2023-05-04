mod configuration;

fn main()
{
    let mut conf = configuration::Config::default();
    conf.init();
}