// bindgen-flags: --whitelist-type Whitelisted --opaque-type Opaque --with-derive-hash --with-derive-partialeq --with-derive-eq -- -std=c++11

// These types are not explicitly whitelisted, but are reachable through the
// opaque type.
class Pupper {};
class Doggo {};
class SuchWow {};

// These types are not whitelisted, and would be reachable through `Opaque` if
// it were not marked opaque, but since it is, there should be no bindings
// generated for them.
class NoBindingsShouldBeGeneratedForMe1 {};
class NoBindingsShouldBeGeneratedForMe2 {};

// Exercise the different kinds of outgoing edges from an opaque type.
class Opaque
  // Base member edge.
  : public NoBindingsShouldBeGeneratedForMe1 {

protected:
    // Field edge.
    NoBindingsShouldBeGeneratedForMe2 field;

    // Constructor edge.
    Opaque(Pupper pup);

    // Inner static variable edge.
    static Doggo MAJESTIC_AF;

    // Method edge.
    SuchWow eleven_out_of_ten();
};

class Whitelisted {
    Opaque some_member;
};
