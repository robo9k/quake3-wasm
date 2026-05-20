#include "g_local.h"

Q_EXPORT intptr_t vmMain( int command, int arg0, int arg1, int arg2, int arg3, int arg4, int arg5, int arg6, int arg7, int arg8, int arg9, int arg10, int arg11  ) {
switch ( command ) {
	case GAME_INIT:
		trap_Error("Hello, world!");
		return 666;
	case GAME_SHUTDOWN:
		return 0;
	}

	return -1;
}
