use std::collections::HashMap;
use std::fmt;

pub const ACC_PUBLIC: u16 = 0x0001; // Declared public; may be accessed from outside its package.
pub const ACC_PRIVATE: u16 = 0x0002; // Declared private; usable only within the defining class.
pub const ACC_PROTECTED: u16 = 0x0004; // Declared protected; may be accessed within subclasses.
pub const ACC_STATIC: u16 = 0x0008; // Declared static

pub const ACC_FINAL: u16 = 0x0010; // Declared final; no subclasses allowed.
pub const ACC_SUPER: u16 = 0x0020; // (Class) Treat superclass methods specially when invoked by the invokespecial instruction.
pub const ACC_VOLATILE: u16 = 0x0040; // (Field) Declared volatile; cannot be cached.
pub const ACC_TRANSIENT: u16 = 0x0080; // (Field) Declared transient; not written or read by a persistent object manager.
pub const ACC_INTERFACE: u16 = 0x0200; // (Class) Is an interface, not a class.
pub const ACC_ABSTRACT: u16 = 0x0400; // (Class) Declared abstract; must not be instantiated.
pub const ACC_SYNTHETIC: u16 = 0x1000; // Declared synthetic; not present in the source code.
pub const ACC_ANNOTATION: u16 = 0x2000; // Declared as an annotation type.
pub const ACC_ENUM: u16 = 0x4000; // Declared as an enum type.

// Method-only constants
pub const ACC_SYNCHRONIZED: u16 = 0x0020; // (Method) Declared synchronized; invocation is wrapped by a monitor use.
pub const ACC_BRIDGE: u16 = 0x0040; // (Method) A bridge, generated by the compiler.
pub const ACC_VARARGS: u16 = 0x0080; // (Method) Declared with variable number of arguments.
pub const ACC_NATIVE: u16 = 0x0100; // (Method) Declared native; implemented in a language other than Java.
pub const ACC_ABSTRACT_M: u16 = 0x0400; // (Method) Declared abstract; no implementation is provided.
pub const ACC_STRICT: u16 = 0x0800; // (Method) Declared strictfp; floating-point mode is FP-strict.

//////////// CONSTANT POOL HANDLING

// CPType constants
pub const CP_UTF8: u8 = 1;
pub const CP_INTEGER: u8 = 3;
pub const CP_FLOAT: u8 = 4;
pub const CP_LONG: u8 = 5;
pub const CP_DOUBLE: u8 = 6;
pub const CP_CLASS: u8 = 7;
pub const CP_STRING: u8 = 8;
pub const CP_FIELDREF: u8 = 9;
pub const CP_METHODREF: u8 = 10;
pub const CP_INTERFACE_METHODREF: u8 = 11;
pub const CP_NAMEANDTYPE: u8 = 12;
pub const CP_METHODHANDLE: u8 = 15;
pub const CP_METHODTYPE: u8 = 16;
pub const CP_INVOKEDYNAMIC: u8 = 18;

#[derive(Clone, Debug)]
pub enum cp_entry {
    utf8 { val: String },
    integer { val: i32 },
    float { val: f32 },
    long { val: i64 },
    double { val: f64 },
    class { idx: u16 },
    string { idx: u16 },
    fieldref { clz_idx: u16, nt_idx: u16 },
    methodref { clz_idx: u16, nt_idx: u16 },
    interface_methodref { clz_idx: u16, nt_idx: u16 },
    name_and_type { name_idx: u16, type_idx: u16 },
}

impl cp_entry {
    pub fn separator(cp_type: u8) -> String {
        match cp_type {
            CP_FIELDREF => ".".to_string(),
            CP_METHODREF => ".".to_string(),
            CP_NAMEANDTYPE => ":".to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct cp_attr {
    name_idx: u16,
}

impl cp_attr {
    pub fn of(name_idx: u16) -> cp_attr {
        cp_attr { name_idx: name_idx }
    }
}

pub fn split_name_desc(name_desc: String) -> (String, String) {
    ("a".to_string(), "b".to_string())
}

//////////// RUNTIME KLASS AND RELATED HANDLING

#[derive(Clone, Debug)]
pub struct ot_klass {
    name: String,
    super_name: String,
    flags: u16,
    cp_entries: Vec<cp_entry>,
    methods: Vec<ot_method>,
    name_desc_lookup: HashMap<String, usize>,
}

impl ot_klass {
    pub fn of(
        klass_name: String,
        super_klass: String,
        flags: u16,
        cp_entries: &Vec<cp_entry>,
        methods: &Vec<ot_method>,
    ) -> ot_klass {
        let mut lookup = HashMap::new();
        let mut i = 0;
        while i < methods.len() {
            let meth = match methods.get(i).clone() {
                Some(val) => val.clone(),
                None => panic!("Error: method {} not found on {}", i, klass_name),
            };
            lookup.insert(meth.get_fq_name_desc().clone(), i);
            i = i + 1;
        }
        dbg!(lookup.clone());
        ot_klass {
            name: klass_name,
            super_name: super_klass,
            flags: flags,
            cp_entries: cp_entries.to_vec(),
            methods: methods.to_vec(),
            name_desc_lookup: lookup,
        }
    }

    // FIXME: Shouldn't this be ot_field for consistency
    pub fn set_static_field(&self, _f: String, _vals: jvm_value) -> () {}

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_super_name(&self) -> String {
        self.super_name.to_owned()
    }

    pub fn get_methods(&self) -> Vec<ot_method> {
        self.methods.clone()
    }

    // NOTE: This is fully-qualified
    pub fn get_method_by_name_and_desc(&self, name_desc: String) -> ot_method {
        dbg!(&self.name_desc_lookup);
        let opt_idx = self.name_desc_lookup.get(&name_desc);
        let idx: usize = match opt_idx {
            Some(value) => value.clone(),
            None => panic!("Error: method {} not found on {}", name_desc, self.name),
        };
        let opt_meth = self.methods.get(idx).clone();
        match opt_meth {
            Some(val) => val.clone(),
            None => panic!("Error: method {} not found on {}", name_desc, self.name),
        }
    }

    pub fn lookup_cp(&self, cp_idx: u16) -> cp_entry {
        let idx = cp_idx as usize;
        // dbg!(&self.cp_entries);
        match self.cp_entries.get(idx).clone() {
            Some(val) => val.clone(),
            None => panic!(
                "Error: No entry found on {} at CP index {}",
                self.name, cp_idx
            ),
        }
    }

    pub fn cp_as_string(&self, i: u16) -> String {
        match self.lookup_cp(i) {
            cp_entry::utf8 { val: s } => s,
            cp_entry::class { idx: utf_idx } => self.cp_as_string(utf_idx),
            cp_entry::methodref { clz_idx, nt_idx } => {
                self.cp_as_string(clz_idx) + "." + &self.cp_as_string(nt_idx)
            }
            cp_entry::name_and_type {
                name_idx: nidx,
                type_idx: tidx,
            } => self.cp_as_string(nidx) + ":" + &self.cp_as_string(tidx),
            _ => panic!(
                "Unimplemented stringify of CP entry found in {} at index {}",
                self.name, i
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ot_method {
    klass_name: String,
    flags: u16,
    name: String,
    name_desc: String,
    name_idx: u16,
    desc_idx: u16,
    code: Vec<u8>,
    attrs: Vec<cp_attr>,
}

impl ot_method {
    pub fn of(
        klass_name: String,
        name: String,
        desc: String,
        flags: u16,
        name_idx: u16,
        desc_idx: u16,
    ) -> ot_method {
        let name_and_desc = name.clone() + ":" + &desc.clone();
        ot_method {
            klass_name: klass_name.to_string(),
            flags: flags,
            name: name.clone(),
            name_desc: name_and_desc,
            attrs: Vec::new(),
            code: Vec::new(),
            // FIXME
            name_idx: desc_idx,
            desc_idx: desc_idx,
        }
    }

    pub fn set_attr(&self, _index: u16, _attr: cp_attr) -> () {}

    pub fn set_code(&mut self, code: Vec<u8>) -> () {
        self.code = code;
    }

    pub fn get_code(&self) -> Vec<u8> {
        self.code.clone()
    }

    pub fn get_klass_name(&self) -> String {
        self.klass_name.clone()
    }

    pub fn get_desc(&self) -> String {
        self.name_desc.clone()
    }

    pub fn get_fq_name_desc(&self) -> String {
        self.klass_name.clone() + "." + &self.name_desc.clone()
    }

    pub fn get_flags(&self) -> u16 {
        self.flags
    }
}

impl fmt::Display for ot_method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.klass_name, self.name_desc)
    }
}

#[derive(Debug)]
pub struct ot_field {
    class_name: String,
    flags: u16,
    name_idx: u16,
    desc_idx: u16,
    name: String,
    attrs: Vec<cp_attr>,
}

impl ot_field {
    pub fn of(
        klass_name: String,
        field_name: String,
        field_flags: u16,
        name: u16,
        desc: u16,
    ) -> ot_field {
        ot_field {
            class_name: klass_name.to_string(),
            // FIXME
            flags: field_flags,
            name_idx: name,
            desc_idx: desc,
            name: field_name,
            attrs: Vec::new(),
        }
    }

    pub fn set_attr(&self, _index: u16, _attr: cp_attr) -> () {}

    pub fn get_name(&self) -> String {
        String::from("")
    }

    pub fn get_klass(&self) -> ot_klass {
        // FIXME DUMMY
        return ot_klass {
            name: "DUMMY_CLASS".to_string(),
            super_name: "DUMMY_SUPER0".to_string(),
            flags: 0,
            cp_entries: Vec::new(),
            methods: Vec::new(),
            name_desc_lookup: HashMap::new(),
        };
    }
}

impl fmt::Display for ot_field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}:{}", self.class_name, self.name, self.desc_idx)
    }
}

//////////// RUNTIME VALUES

#[derive(Copy, Clone)]
pub enum jvm_value {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char { val: char },
    ObjRef { val: ot_obj },
}

impl jvm_value {
    fn name(&self) -> char {
        match *self {
            jvm_value::Boolean { val: _ } => 'Z',
            jvm_value::Byte { val: _ } => 'B',
            jvm_value::Short { val: _ } => 'S',
            jvm_value::Int { val: _ } => 'I',
            jvm_value::Long { val: _ } => 'J',
            jvm_value::Float { val: _ } => 'F',
            jvm_value::Double { val: _ } => 'D',
            jvm_value::Char { val: _ } => 'C',
            jvm_value::ObjRef { val: _ } => 'A',
        }
    }
}

impl fmt::Display for jvm_value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            jvm_value::Boolean { val: v } => write!(f, "{}", v),
            jvm_value::Byte { val: v } => write!(f, "{}", v),
            jvm_value::Short { val: v } => write!(f, "{}", v),
            jvm_value::Int { val: v } => write!(f, "{}", v),
            jvm_value::Long { val: v } => write!(f, "{}", v),
            jvm_value::Float { val: v } => write!(f, "{}", v),
            jvm_value::Double { val: v } => write!(f, "{}", v),
            jvm_value::Char { val: v } => write!(f, "{}", v),
            jvm_value::ObjRef { val: v } => write!(f, "{}", v),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ot_obj {
    mark: u64,
    klassid: u32, // FIXME: This should become a pointer at some point
}

impl ot_obj {
    pub fn put_field(&self, _f: ot_field, _val: jvm_value) -> () {}

    pub fn get_null() -> ot_obj {
        ot_obj {
            mark: 0u64,
            klassid: 0u32,
        }
    }

    pub fn is_null(&self) -> bool {
        if self.mark == 0u64 && self.klassid == 0u32 {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for ot_obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MarK: {} ; Klass: {}", self.mark, self.klassid)
    }
}

//////////// RUNTIME STACKS AND LOCAL VARS

pub struct interp_eval_stack {
    stack: Vec<jvm_value>,
}

impl interp_eval_stack {
    pub fn of() -> interp_eval_stack {
        interp_eval_stack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: jvm_value) -> () {
        let s = &mut self.stack;
        s.push(val);
    }

    pub fn pop(&mut self) -> jvm_value {
        let s = &mut self.stack;
        match s.pop() {
            Some(value) => value,
            None => panic!("pop() on empty stack"),
        }
    }

    pub fn aconst_null(&mut self) -> () {
        self.push(jvm_value::ObjRef {
            val: ot_obj::get_null(),
        });
    }

    pub fn iconst(&mut self, v: i32) -> () {
        self.push(jvm_value::Int { val: v });
    }

    pub fn iadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(jvm_value::Int { val: i1 + i2 });
    }

    pub fn isub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(jvm_value::Int { val: i1 - i2 });
    }
    pub fn imul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(jvm_value::Int { val: i1 * i2 });
    }

    pub fn irem(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(jvm_value::Int { val: i2 % i1 });
    }
    pub fn ixor(&self) -> () {}
    pub fn idiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(jvm_value::Int { val: i2 / i1 });
    }
    pub fn iand(&self) -> () {}
    pub fn ineg(&mut self) -> () {
        let i1 = match self.pop() {
            jvm_value::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(jvm_value::Int { val: -i1 });
    }
    pub fn ior(&self) -> () {}

    pub fn dadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(jvm_value::Double { val: i1 + i2 });
    }
    pub fn dsub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(jvm_value::Double { val: i1 - i2 });
    }
    pub fn dmul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            jvm_value::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            jvm_value::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(jvm_value::Double { val: i1 * i2 });
    }

    pub fn dconst(&mut self, v: f64) -> () {
        self.push(jvm_value::Double { val: v });
    }

    pub fn i2d(&self) -> () {}
    pub fn dup(&mut self) -> () {
        let i1 = self.pop();
        self.push(i1.to_owned());
        self.push(i1.to_owned());
    }
    pub fn dupX1(&mut self) -> () {
        let i1 = self.pop();
        let i2 = self.pop();
        self.push(i1);
        self.push(i2);
        self.push(i1);
    }
}

pub struct interp_local_vars {
    lvt: [jvm_value; 256],
}

impl interp_local_vars {
    pub fn of() -> interp_local_vars {
        interp_local_vars {
            lvt: [jvm_value::Int { val: 0 }; 256],
        }
    }

    pub fn load(&self, idx: u8) -> jvm_value {
        self.lvt[idx as usize]
    }

    pub fn store(&mut self, idx: u8, val: jvm_value) -> () {
        self.lvt[idx as usize] = val
    }

    pub fn iinc(&mut self, idx: u8, incr: u8) -> () {
        match self.lvt[idx as usize] {
            jvm_value::Int { val: v } => {
                self.lvt[idx as usize] = jvm_value::Int { val: v + 1 };
            }
            _ => panic!("Non-integer value encountered in IINC of local var {}", idx),
        }
    }
}

//////////// SHARED RUNTIME STRUCTURES

pub struct vm_context {
    heap: shared_simple_heap,
    repo: shared_klass_repo,
}

impl vm_context {
    pub fn of() -> vm_context {
        vm_context {
            heap: shared_simple_heap {},
            repo: shared_klass_repo::new(),
        }
    }

    pub fn get_repo(&mut self) -> &mut shared_klass_repo {
        &mut self.repo
    }

    pub fn get_heap(&mut self) -> &mut shared_simple_heap {
        &mut self.heap
    }

    pub fn allocate_obj(&mut self, klass: &ot_klass) -> ot_obj {
        self.heap.allocate_obj(klass)
    }
}

#[derive(Clone, Debug)]
pub struct shared_klass_repo {
    klass_lookup: HashMap<String, ot_klass>,
}

impl shared_klass_repo {
    pub fn new() -> shared_klass_repo {
        shared_klass_repo {
            klass_lookup: HashMap::new(),
        }
    }

    pub fn lookup_field(&self, _klass_name: String, _idx: u16) -> ot_field {
        // FIXME DUMMY
        ot_field::of(
            "DUMMY_KLASS".to_string(),
            "DUMMY_FIELD".to_string(),
            0,
            1,
            2,
        )
    }

    pub fn lookup_method_exact(&self, klass_name: &String, fq_name_desc: String) -> ot_method {
        match self.klass_lookup.get(klass_name) {
            Some(k) => k.get_method_by_name_and_desc(fq_name_desc),
            None => panic!("No klass called {} found in repo", klass_name),
        }
    }

    pub fn lookup_method_virtual(&self, _klass_name: &String, _idx: u16) -> ot_method {
        // FIXME DUMMY
        ot_method::of(
            "DUMMY_KLASS".to_string(),
            "DUMMY_METH".to_string(),
            "DUMMY_DESC".to_string(),
            0,
            1,
            2,
        )
    }

    // FIXME SIG
    pub fn lookup_klass(&self, klass_name: String) -> &ot_klass {
        match self.klass_lookup.get(&klass_name) {
            Some(value) => value,
            None => panic!("Error looking up {} - no value returned", klass_name),
        }

        // // FIXME DUMMY
        // ot_klass {
        //     name: klass_name.to_string(),
        //     super_name: "DUMMY_SUPER".to_string(),
        //     flags: 0,
        //     cp_entries: Vec::new(),
        //     methods: Vec::new(),
        //     name_desc_lookup: HashMap::new(),
        // }
    }

    pub fn add_klass(&mut self, k: ot_klass) -> () {
        self.klass_lookup.insert(k.get_name().clone(), k.clone());
    }
}

pub struct shared_simple_heap {}

impl shared_simple_heap {
    pub fn allocate_obj(&self, klass: &ot_klass) -> ot_obj {
        // FIXME
        ot_obj::get_null()
    }
}
