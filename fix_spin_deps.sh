#!/bin/bash

# Fix spin-sdk dependencies in all tools
# Move spin-sdk from target-specific to main dependencies

echo "Fixing spin-sdk dependencies in all tools..."

# Find all tools with target-specific spin-sdk dependencies
find tools -name "Cargo.toml" | while read cargo_file; do
    if grep -q "target.*wasm32.*dependencies" "$cargo_file" && grep -A 5 "target.*wasm32.*dependencies" "$cargo_file" | grep -q "spin-sdk"; then
        echo "Fixing $cargo_file..."
        
        # Create a temporary file
        temp_file=$(mktemp)
        
        # Process the file
        awk '
        BEGIN { in_target_section = 0; spin_line = "" }
        
        # Detect start of target section
        /^\[target\./ { in_target_section = 1 }
        
        # Detect start of any other section
        /^\[/ && !/^\[target\./ { 
            if (in_target_section && spin_line != "") {
                # Add spin-sdk to main dependencies section if we found it in target section
                print spin_line
                spin_line = ""
            }
            in_target_section = 0 
        }
        
        # If in target section and this is spin-sdk line, store it
        in_target_section && /^spin-sdk/ { 
            spin_line = $0
            next  # Skip this line
        }
        
        # For dependencies section, add spin-sdk if we have one stored
        /^\[dependencies\]/ {
            print $0
            if (spin_line != "") {
                print spin_line
                spin_line = ""
            }
            next
        }
        
        # Print all other lines
        { print $0 }
        
        END {
            # If we still have a spin_line and no dependencies section was found, 
            # we need to add it after ftl-sdk line
            if (spin_line != "") {
                print "# Note: spin-sdk needs to be moved to [dependencies] section"
            }
        }
        ' "$cargo_file" > "$temp_file"
        
        # Replace the original file
        mv "$temp_file" "$cargo_file"
        echo "Fixed $cargo_file"
    fi
done

echo "Done fixing spin-sdk dependencies!"