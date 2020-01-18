
import neworder as no

def func():
  print("func")
  return no.x + no.foo(3) #sum_as_string(1,2)

if __name__ == "__main__":
  func()