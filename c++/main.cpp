#include <iostream>
#include <vector>

using std::cout;
using std::vector;

int global_id = 0;
void reset_global_counter(char const *msg) {
  cout << "\n\n" << msg << "\n";
  global_id = 0;
}

struct Counter {
  int c = 0, id;
  Counter() { id = ++global_id; }
  Counter(Counter && o) { id = ++global_id; c = o.c +1; }
  Counter(const Counter & o) { id = ++global_id; c = o.c +1; }
  Counter& operator=(Counter && o)      { c = o.c +1; return *this; }
  Counter& operator=(const Counter & o) { c = o.c +1; return *this; }
  ~Counter() { cout << "c(" << id << "): " << c << "\n"; }

  void dummy() { /* Shut up compiler */ }
};

int main() {
  {
    reset_global_counter("Eins");
    Counter c{};
  }

  {
    reset_global_counter("Zwei");
    vector<Counter> v{ Counter{} };

    cout << "begin auto loop\n";
    for (auto e : v) {
      e.dummy();
    }

    cout << "begin decltype(auto) loop\n";
    for (decltype(auto) e : v) {
      e.dummy();
    }


    cout << "vector destroy\n";
  }
}
