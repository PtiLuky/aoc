def fuelFor(mass: int):
    moduleFuel = int(int(mass)/3)-2
    return 0 if moduleFuel <= 0 else moduleFuel + fuelFor(moduleFuel)

totalFuel=0
with open('input.txt') as f:
  for mass in f:
    totalFuel += fuelFor(mass)

print(totalFuel)

