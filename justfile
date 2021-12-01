check-all:
    cargo check

init name:
    cd days && cargo generate --path ../_template --name day-{{name}}

check name:
    cd days/day-{{name}} && cargo check

fmt name:
    cd days/day-{{name}} && cargo fmt

clippy name: 
    cd days/day-{{name}} && cargo clippy --fix

run name: (check name) (fmt name) (clippy name)
    cargo run day-{{name}}

clean:
    cargo clean

done name: (run name)
    git add _template days/day-{{name}}ss
    git cm "Done with {{name}}"
    git push