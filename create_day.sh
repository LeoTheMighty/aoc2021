# Create a module
# "NAME SPECIFIED" - (make sure we don't overwrite one accidentally)
#   - mod.rs (just contains "pub mod solution")
#   - solution.rs (contains "pub fn print_solution() {}")

echo "Creating solution directory $1"
if [ ! -d "src/$1" ]; then
  # Control will enter here if $DIRECTORY doesn't exist.
  mkdir "src/$1"
  echo "pub mod solution;" >> "src/$1/mod.rs"
  echo "pub fn print_solution() {
  println!(\"Hello world!\");
}" >> "src/$1/solution.rs"
else
  echo "Directory already exists, aborting..."
fi