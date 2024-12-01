i = 0

File.readlines('test.txt').each do |line|

    first,second = line.split(',')
    
    f1,t1 = first.split('-')
    f2,t2 = second.split('-')

    tmp1 = Array.new
    tmp2 = Array.new

    (f1.to_i..t1.to_i).each do |f|
        tmp1.append(f.to_i)
    end
    

    (f2.to_i..t2.to_i).each do |f|
        tmp2.append(f.to_i)
    end


    if (!(tmp1 & tmp2).empty?)
        i = i + 1
    end
end


puts i