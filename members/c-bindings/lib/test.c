#include "test.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int ret19() {
	return 19;
}

char* structInfo(MyStruct *st) {
		char* buf = malloc(1024*sizeof(char));
		/* snprintf(buf, sizeof(buf), "foo: %d\n", st->foo); */
		/* snprintf(buf, sizeof(buf), "bar: %c", st->bar); */
		sprintf(buf,"Infos about the struct:\nfoo:\t%d\nbar:\t%c\n\ngreetings from C", st->foo, st->bar);

		return buf;
}
