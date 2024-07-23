extern crate embed_resource;

fn main() -> std::io::Result<()> {
    {
        embed_resource::compile("res/resource.rc",embed_resource::NONE);
    }
    return Ok(());
}