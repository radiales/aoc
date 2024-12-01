stack = Array.new
File.readlines('input6.txt').each do |line|

    i = 0

    array = line.split('')

    array.each do |x|

        0..14.times do |x|
            stack.push(array[i + x])
        end


        if stack.uniq == stack
            puts stack
            puts i + 14
            break
        end

        stack.clear
        i += 1
      
    end
  
end
