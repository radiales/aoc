stack = Array.new
File.readlines('input6.txt').each do |line|

    i = 0
    j = 0

    array = line.split('')

    array.each do |x|
        j += 1

       stack.push(array[i])
       stack.push(array[i + 1])
       stack.push(array[i + 2])
       stack.push(array[i + 3])

        if stack.uniq == stack
            puts j + 3
            break
        end

        stack.clear
        i += 1
      
    end
  
end
