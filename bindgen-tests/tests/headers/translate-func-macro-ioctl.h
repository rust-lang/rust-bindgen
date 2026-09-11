// bindgen-flags: --translate-function-macros

// Simulates the real linux ioctl pattern where _IOC_TYPECHECK wraps sizeof
// and _IOR/_IOW pass type params through it.

#define _IOC_NRSHIFT    0
#define _IOC_TYPESHIFT  8
#define _IOC_SIZESHIFT  16
#define _IOC_DIRSHIFT   30

#define _IOC_NONE  0U
#define _IOC_READ  2U

#define _IOC(dir,type,nr,size) \
    (((dir)  << _IOC_DIRSHIFT) | \
     ((type) << _IOC_TYPESHIFT) | \
     ((nr)   << _IOC_NRSHIFT) | \
     ((size) << _IOC_SIZESHIFT))

#define _IOC_TYPECHECK(t) (sizeof(t))

#define _IO(type,nr)           _IOC(_IOC_NONE,(type),(nr),0)
#define _IOR(type,nr,argtype)  _IOC(_IOC_READ,(type),(nr),(_IOC_TYPECHECK(argtype)))
