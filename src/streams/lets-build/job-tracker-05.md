Let’s Build: A Job Tracker - Part 5
===================================

What’s wrong with what we have:
- StoreContext is in UI for some reason
- Lots of _very_ similar repeated code
- Adding new types requires a lot of boilerplate (eg, Values aren’t really any
different from Flags or Roles in the way they are stored and recalled)

Storable objects have specific traits to allow them to optionally be:
- stored (needs an ID)
- recalled by ID (needs an ID)
- recalled by name (needs a name)
- recalled by company ID (needs a company name)

This is going to be a _lot_ of traits which need to be inscope, we should use a
prelude

A store for a generic Storable can use the traits to apply logic. There’s no
real difference between a flag and a role in terms of how its stored and
recalled, therefore there should be no difference in logic. 

So we can have something like:
- `trait StoreById<T> where T: GetId`
- `trait RecallById<T> where T: GetId`
- `trait RecallByName<T> where T: GetName`

We can create generic stores for storables that match certain traits, then
build those generics together. Eg, a CompanyStore can be a trait that combines
- `StoreById<Company>`
- `RecallById<Company> `
- `RecallByName<Company>`

If we have a store that already implements those traits, then we get it for free

A collection of stores should be able to determine which store an object
belongs to by its type. <- This will be the harder part, may need phantom data
in Store types 🤔

A collection of stores can implement the trait CompanyStore but we may be able
to make it even more generic than that if we can go from generic type to
property in stores

One way to do this might be to have a trait called `HasStore<T> -> S` and then
implement:
- `HasStore<Company> -> impl CompanyStore`
- `HasStore<Role> -> impl RoleStore`
- Etc

Suggested structure:

```text
/storable
  /property
    mod.rs
    has_id.rs
    has_name.rs
    has_company.rs
  /object
    mod.rs
    company.rs
    role.rs
    flag.rs
    value.rs
/store
  /property
    mod.rs
    store.rs (probably both store and recall by id)
    recall_by_name.rs
    recall_by_company.rs
  /medium
    mod.rs
    stub.rs
    json.rs
  /object_store
    (probably collections of compound traits?)
/stores
  mod.rs // HasStore<T> trait here?
  /mediums
    mod.rs
````
