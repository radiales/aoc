# Define a method that takes an array and an index
def split_array_at_index(array, index)
        # Split the array into two sub-arrays using the 'slice' method,
        # and return the sub-arrays
        return array.slice(0, index), array.slice(index + 1, array.length)
      end
      
      # Define an array
      array = [1, 2, 3, 4, 5]
      
      # Split the array at index 2 without the element at index 2
      first, second = split_array_at_index(array, 2)
      
      # Print the resulting sub-arrays
      puts first.inspect
      puts second.inspect
      