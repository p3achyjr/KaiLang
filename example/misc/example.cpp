class A
{       // The class
public: // Access specifier
  A(int a, int b) : a(a), b(b) {}
  int a; // Attribute (int variable)
  int b; // Attribute (string variable)
};

int main()
{
  auto cls = new A(5, 6);
  return (*cls).a + (*cls).b;
}