######## SGX SDK Settings ########
RUST_SGX_SDK ?= ../../..
SGX_SDK ?= /opt/intel/sgxsdk

App_Include_Paths := -I$(SGX_SDK)/include -I$(RUST_SGX_SDK)/edl
App_C_Flags := -O2 -fPIC -Wno-attributes $(App_Include_Paths)

SQLite_Object := sqlite3.o
SQLite_i := sqlite3.i

SQLite_OCall_Object := ocall.o
SQLite_OCall_i_Object := ocall_interface.o
SQLite_OCall_i := ocall_interface.i

.PHONY: all
all: $(SQLite_Object) $(SQLite_OCall_Object) $(SQLite_OCall_i_Object)

SQLite_thread_Flag += -DSQLITE_THREADSAFE=0
SQLite_Object_Flags += -m64 -O2 -nostdinc \
	-fvisibility=hidden -fpie -ffunction-sections -fdata-sections -fstack-protector-strong -IEnclave \
	-I$(SGX_SDK)/include \
	-I$(SGX_SDK)/include/tlibc \
	-I$(SGX_SDK)/include/libcxx \
	$(SQLite_thread_Flag)

ifneq ($(SQLite_Object),$(wildcard $(SQLite_Object)))
$(SQLite_Object): $(SQLite_i)
	@echo "build sqlite3 ..."
	@$(CC) $(SQLite_Object_Flags) -c $< -o $@
	@rm -f $<

$(SQLite_OCall_Object): ocall/ocalls.c
	@echo "build sqlite3 ocall interface ..."
	@$(CC) $(App_C_Flags) -c $< -o $@

$(SQLite_i): sqlite3.c
	@$(CC) -I$(SGX_SDK)/include $(SQLite_thread_Flag) -E $< -o $@
endif

ifneq ($(SQLite_OCall_i_Object),$(wildcard $(SQLite_OCall_i_Object)))
$(SQLite_OCall_i_Object): $(SQLite_OCall_i)
	@echo "build sqlite3 ocall interface ..."
	@$(CC) $(SQLite_Object_Flags) -c $< -o $@
	@rm -f $<

$(SQLite_OCall_i): ocall/ocall_interface.c
	@$(CC) -I$(SGX_SDK)/include $(SQLite_thread_Flag) -E $< -o $@
endif

.PHONY: clean
clean:
	@rm -f *.i *.o
