require "set"

i = 0

File.readlines('input4.txt').each do |line|

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
    
    a1 = tmp1.to_set
    a2 = tmp2.to_set

    if (a1.subset?(a2))
        puts "#{tmp1} SUBSET #{tmp2}"
        i = i + 1

    elsif (a2.subset?(a1))
        puts "#{tmp2} SUBSET #{tmp1}"
        i = i + 1

    else
        i = i
    end

end


puts i