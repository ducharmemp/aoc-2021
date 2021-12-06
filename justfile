check-all:
    cargo check

init name:
    cd days && cargo generate --path ../_template --name day-{{name}}

check name:
    cd days/day-{{name}} && cargo check

fmt name:
    cd days/day-{{name}} && cargo fmt

clippy name: 
    cd days/day-{{name}} && cargo clippy --fix --allow-dirty --allow-staged

run name: (check name) (fmt name) (clippy name)
    cargo run -p day-{{name}} --release

clean:
    cargo clean

done name: (run name)
    git add _template days/day-{{name}} justfile
    git cm "Done with {{name}}"
    git push

docs:
    cargo watch -s 'cargo doc && http target/doc'