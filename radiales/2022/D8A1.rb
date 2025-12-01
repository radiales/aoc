# Define a method that takes an array and an index
def split_array_at_index(array, index)
    # Split the array into two sub-arrays using the 'slice' method,
    # and return the sub-arrays
    return array.slice(0, index ), array.slice(index + 1, array.length)
  end

matrix = []
visible = 0

length = 0
width = 0

File.readlines('input8.txt', chomp: true).each do |line|
  matrix.push(line.split('').map(&:to_i))

  length += 1
end
width = matrix[0].length


visible = (2 * length + 2 * width) - 4 # All outer trees

length.times do |l|
    if l == 0 || l == length - 1
        next
    end
    width.times do |w|
        if w == 0 || w == width - 1
            next
        end

        left, right = split_array_at_index(matrix[l], w)
        left2, right2 = split_array_at_index((matrix.transpose)[w], l)


        if (matrix[l][w] > left.max) || (matrix[l][w] > right.max)
            puts " left MAX #{left} right MAX #{right}"
            visible += 1
            puts "COORDS #{l} #{w} NORM"
        elsif (matrix.transpose[w][l] > left2.max) || (matrix.transpose[w][l] > right2.max)
            puts " left2 MAX #{left} right2 MAX #{right}"

            visible += 1
            puts "COORDS #{l} #{w} TRANS"
        end


    end
end

puts visible








