def split_array_at_index(array, index)
    return array.slice(0, index ).reverse, array.slice(index + 1, array.length)
end

def get_score(array, val)
    if array == []
        return 1
    end

    score = 0
    endcheck= 0
    array.each do |a|
      if a < val && endcheck == 0
        score += 1
      else
        score += 1
        break
      end
    end
    score.to_i
end
  


matrix = []
length = 0
width = 0
endval = 0


File.readlines('input8.txt', chomp: true).each do |line|
  matrix.push(line.split('').map(&:to_i))

  length += 1
end
width = matrix[0].length

length.times do |l|
    if l == 0 || l == length - 1
        next
    end
    width.times do |w|
        if w == 0 || w == width - 1
            next
        end

        left, right = split_array_at_index(matrix[l], w)
        up, down = split_array_at_index((matrix.transpose)[w], l)

        tmpval = get_score(left, matrix[l][w]) * 
                 get_score(right, matrix[l][w]) * 
                 get_score(up, matrix[l][w]) * 
                 get_score(down, matrix[l][w])


        # if l==3 && w==3

        #     puts "#{up.join(' ')} Up array"
        #     puts "#{left.join(' ')} Left array"	
        #     puts "#{right.join(' ')} Right array"	
        #     puts "#{down.join(' ')} Down array"

        #     puts" ----"

        #     puts get_score(up, matrix[l][w])
        #     puts get_score(left, matrix[l][w])
        #     puts get_score(right, matrix[l][w])
        #     puts get_score(down, matrix[l][w])
        #     #puts tmpval
        # end

        
            if tmpval > endval
                endval = tmpval
            end
            
            
    end
end
puts endval


