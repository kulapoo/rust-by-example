struct A;
struct S(A);

struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}


fn generic2<T, U>(_a: T, _b: U) {}

fn main() {
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.


    generic::<char>(SGen('a'));

    generic2::<i32, &str>(5, "hahah")
}