import example

tire = example.create_random_tire()
print(f'Tire pressure: {tire.pressure:.2f}')
print(f'Tire material: {tire.material}')
print(f'Tire size: {tire.size.width:.2f} width, {tire.size.height:.2f} height')
example.format_wheel_identifier([77, 42, 69])
example.format_size_of_wheels([77, 42, 69])
example.format_size_of_wheels([77, 42, 69, 30])
example.func_with_no_return()
