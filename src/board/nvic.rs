// NVIC Description - is on pg 529
// NVIC Registers (Programming Manual) - is on 178

/* Nested vectored interrupt controller (NVIC) */

#[repr(C)]
pub struct NVICReg {
	pub iser:		[u32;  8],		/* Interrupt Set-Enable register */
	pub reserved0:	[u32; 24],
	pub icer:		[u32;  8], 		/* Interrupt Clear-Enable register */
	pub reserved1:	[u32; 24],
	pub ispr:		[u32;  8],    	/* Interrupt Set-Pending Registers */
	pub reserved2:	[u32; 24],
	pub icpr:		[u32;  8],	    /* Interrupt Clear-Pending Registers */
	pub reserved3:	[u32; 24],
	pub iabr:		[u32;  8], 		/* Interrupt Active Bit Registers */
    pub reserved4:	[u32; 56],
	pub ipr:		[u8;  60],	    /* Interrupt Priority Registers */
}