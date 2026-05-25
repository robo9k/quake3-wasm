#include "g_local.h"
__attribute__((import_name("syscall"))) intptr_t syscall(intptr_t arg, ...);
//static intptr_t (QDECL *syscall)( intptr_t arg, ... ) = (intptr_t (QDECL *)( intptr_t, ...))-1;
/*
Q_EXPORT void dllEntry( intptr_t (QDECL *syscallptr)( intptr_t arg,... ) ) {
	syscall = syscallptr;
}
*/
void trap_Error( const char *text )
{
	syscall( G_ERROR, text );
}
