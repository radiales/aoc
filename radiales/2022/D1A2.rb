tmp = 0
calories=Array.new

File.readlines('input.txt').each do |line|

  tmp = line.to_i + tmp


  if line.to_s.strip.empty?
    calories.append(tmp)
    tmp = 0
  end

end

first = calories.max
calories.delete(first)

second = calories.max
calories.delete(second)

third = calories.max
calories.delete(third)

puts(first+second+third)
