mkdir $1
mkdir $1/inputs
mkdir $1/rust
mkdir $1/rust/solutions

touch $1/rust/solutions/mod.rs
cp scripts/main_template.rs $1/rust/main.rs
sed -i s/XXXX/$1/ $1/rust/main.rs
