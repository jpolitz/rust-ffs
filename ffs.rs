#[allow(dead_code)]

const MAXMNTLEN: usize = 468;
const MAXVOLLEN: usize = 32;
/*
 * defined in OpenBSD's code as:
 *     (128 / sizeof(void *)) - 4
 * We would use:
 *     (128 / sys::size_of::<usize>()) - 4
 * But will hard-code for I32LP64 for now.
 */
const NOCSPTRS: usize = (128 / 8) - 4;
const FSMAXSNAP: usize = 20;

/*
 * Per cylinder group information; summarized in blocks allocated
 * from first cylinder group data blocks.  These blocks have to be
 * read in from fs_csaddr (size fs_cssize) in addition to the
 * super block.
 */
#[allow(dead_code)]
struct CSum {
    cs_ndir: i32,        /* number of directories */
    cs_nbfree: i32,        /* number of free blocks */
    cs_nifree: i32,        /* number of free inodes */
    cs_nffree: i32,        /* number of free frags */
}

#[allow(dead_code)]
struct CSumTotal {
    cs_ndir: i64,        /* number of directories */
    cs_nbfree: i64,        /* number of free blocks */
    cs_nifree: i64,        /* number of free inodes */
    cs_nffree: i64,        /* number of free frags */
    cs_spare: [i64; 4],        /* future expansion */
}

/*
 * Super block for an FFS file system.
 */
#[allow(dead_code)]
struct FS<'a> {
    fs_firstfield: i32,        /* historic file system linked list, */
    fs_unused_1: i32,        /*     used for incore super blocks */
    fs_sblkno: i32,        /* addr of super-block / frags */
    fs_cblkno: i32,        /* offset of cyl-block / frags */
    fs_iblkno: i32,        /* offset of inode-blocks / frags */
    fs_dblkno: i32,        /* offset of first data / frags */
    fs_cgoffset: i32,        /* cylinder group offset in cylinder */
    fs_cgmask: i32,        /* used to calc mod fs_ntrak */
    fs_ffs1ime: i32,        /* last time written */
    fs_ffs1_size: i32,        /* # of blocks in fs / frags */
    fs_ffs1_dsize: i32,        /* # of data blocks in fs */
    fs_ncg: i32,        /* # of cylinder groups */
    fs_bsize: i32,        /* size of basic blocks / bytes */
    fs_fsize: i32,        /* size of frag blocks / bytes */
    fs_frag: i32,        /* # of frags in a block in fs */
/* these are configuration parameters */
    fs_minfree: i32,        /* minimum percentage of free blocks */
    fs_rotdelay: i32,        /* # of ms for optimal next block */
    fs_rps: i32,        /* disk revolutions per second */
/* these fields can be computed from the others */
    fs_bmask: i32,        /* ``blkoff'' calc of blk offsets */
    fs_fmask: i32,        /* ``fragoff'' calc of frag offsets */
    fs_bshift: i32,        /* ``lblkno'' calc of logical blkno */
    fs_fshift: i32,        /* ``numfrags'' calc # of frags */
/* these are configuration parameters */
    fs_maxcontig: i32,        /* max # of contiguous blks */
    fs_maxbpg: i32,        /* max # of blks per cyl group */
/* these fields can be computed from the others */
    fs_fragshift: i32,        /* block to frag shift */
    fs_fsbtodb: i32,        /* fsbtodb and dbtofsb shift constant */
    fs_sbsize: i32,        /* actual size of super block */
    fs_csmask: i32,        /* CSum block offset (now unused) */
    fs_csshift: i32,        /* CSum block number (now unused) */
    fs_nindir: i32,        /* value of NINDIR */
    fs_inopb: i32,        /* inodes per file system block */
    fs_nspf: i32,        /* DEV_BSIZE sectors per frag */
/* yet another configuration parameter */
    fs_optim: i32,        /* optimization preference, see below */
/* these fields are derived from the hardware */
    fs_npsect: i32,        /* DEV_BSIZE sectors/track + spares */
    fs_interleave: i32,        /* DEV_BSIZE sector interleave */
    fsrackskew: i32,        /* sector 0 skew, per track */
/* fs_id takes the space of the unused fs_headswitch and fsrkseek fields */
    fs_id: [i32; 2],        /* unique filesystem id */
/* sizes determined by number of cylinder groups and their sizes */
    fs_ffs1_csaddr: i32,    /* blk addr of cyl grp summary area */
    fs_cssize: i32,        /* cyl grp summary area size / bytes */
    fs_cgsize: i32,        /* cyl grp block size / bytes */
/* these fields are derived from the hardware */
    fs_ntrak: i32,        /* tracks per cylinder */
    fs_nsect: i32,        /* DEV_BSIZE sectors per track */
    fs_spc: i32,        /* DEV_BSIZE sectors per cylinder */
/* this comes from the disk driver partitioning */
    fs_ncyl: i32,        /* cylinders in file system */
/* these fields can be computed from the others */
    fs_cpg: i32,        /* cylinders per group */
    fs_ipg: i32,        /* inodes per group */
    fs_fpg: i32,        /* blocks per group * fs_frag */
/* this data must be re-computed after crashes */
    fs_ffs1_cstotal: CSum,    /* cylinder summary information */
/* these fields are cleared at mount time */
    fs_fmod: i8,        /* super block modified flag */
    fs_clean: i8,        /* file system is clean flag */
    fs_ronly: i8,        /* mounted read-only flag */
    fs_ffs1_flags: i8,        /* see FS_ below */
    fs_fsmnt: [u8; MAXMNTLEN],    /* name mounted on */
    fs_volname: [u8; MAXVOLLEN],    /* volume name */
    fs_swuid: u64,        /* system-wide uid */
    fs_pad: i32,        /* due to alignment of fs_swuid */
/* these fields retain the current block allocation info */
    fs_cgrotor: i32,        /* last cg searched */
    fs_ocsp: [usize; NOCSPTRS],    /* padding; was list of fs_cs buffers */
    fs_contigdirs: &'a u8,    /* # of contiguously allocated dirs */
    fs_csp: &'a CSum,        /* cg summary info buffer for fs_cs */
    fs_maxcluster: &'a i32,        /* max cluster in each cyl group */
    fs_active: &'a u8,        /* reserved for snapshots */
    fs_cpc: i32,        /* cyl per cycle in postbl */
/* this area is only allocated if fs_ffs1_flags & FS_FLAGS_UPDATED */
    fs_maxbsize: i32,           /* maximum blocking factor permitted */
    fs_spareconf64: [i64; 17],    /* old rotation block list head */
    fs_sblockloc: i64,          /* offset of standard super block */
    fs_cstotal: CSumTotal,  /* cylinder summary information */
    fsime: i64,               /* time last written */
    fs_size: i64,               /* number of blocks in fs */
    fs_dsize: i64,              /* number of data blocks in fs */
    fs_csaddr: i64,             /* blk addr of cyl grp summary area */
    fs_pendingblocks: i64,      /* blocks in process of being freed */
    fs_pendinginodes: i32,      /* inodes in process of being freed */
    fs_snapinum: [i32; FSMAXSNAP],/* space reserved for snapshots */
/* back to stuff that has been around a while */
    fs_avgfilesize: i32,    /* expected average file size */
    fs_avgfpdir: i32,        /* expected # of files per directory */
    fs_sparecon: [i32; 26],    /* reserved for future constants */
    fs_flags: u32,        /* see FS_ flags below */
    fs_fscktime: i32,        /* last time fsck(8)ed */
    fs_contigsumsize: i32,    /* size of cluster summary array */ 
    fs_maxsymlinklen: i32,    /* max length of an internal symlink */
    fs_inodefmt: i32,        /* format of on-disk inodes */
    fs_maxfilesize: u64,    /* maximum representable file size */
    fs_qbmask: i64,        /* ~fs_bmask - for use with quad size */
    fs_qfmask: i64,        /* ~fs_fmask - for use with quad size */
    fs_state: i32,        /* validate fs_clean field */
    fs_postblformat: i32,    /* format of positional layout tables */
    fs_nrpos: i32,        /* number of rotational positions */
    fs_postbloff: i32,        /* (u16) rotation block list head */
    fs_rotbloff: i32,        /* (u8) blocks for each rotation */
    fs_magic: i32,        /* magic number */
    fs_space: [u8; 1],        /* list of blocks for each rotation */
/* actually longer */
}
