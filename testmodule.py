
import neworder as no

def func():
  no.log("func")
  no.log(3)
  no.log(1.2)
  return no.name() #+ 3 #no.foo(3) #sum_as_string(1,2)

if __name__ == "__main__":
  func()