import example

tire = example.create_random_tire()
print(f'Tire pressure: {tire.pressure}')
print(f'Tire material: {tire.material}')
print(f'Tire size: {tire.size.width}w, {tire.size.height}h')
print(f'Tire size: {tire.size.width:.2f} width, {tire.size.height:.2f} height')

example.func_with_no_return()
