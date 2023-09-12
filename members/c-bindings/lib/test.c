#include "test.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int ret19() {
	return 19;
}

char* structInfo(MyStruct *st) {
		char* buf = malloc(1024*sizeof(char));
		sprintf(buf,
				"Infos about the struct:\n"
				"\tfoo:\t%d\n"
				"\tbar:\t%c\n"
				"\n"
				"greetings from C"
				,
				st->foo,
				st->bar
		);

		return buf;
}
