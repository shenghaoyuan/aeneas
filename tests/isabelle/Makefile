# Makefile for tests/isabelle
# This file orchestrates running 'make' in each subdirectory.

# Find all subdirectories (test cases), excluding templates and build artifacts
ALL_DIRS ?= $(filter-out %.template, $(filter-out Makefile%, $(filter-out output%, $(wildcard *))))

VERIFY_DIRS = $(addprefix verif-,$(ALL_DIRS))
CLEAN_DIRS = $(addprefix clean-,$(ALL_DIRS))
COPY_MAKEFILES = $(addprefix copy-makefile-,$(ALL_DIRS))
GEN_ISABELLE_ROOTS = $(addprefix gen-isabelle-root-,$(ALL_DIRS))

.PHONY: all
all: prepare-projects verify

.PHONY: prepare-projects
prepare-projects: $(COPY_MAKEFILES) $(GEN_ISABELLE_ROOTS)

.PHONY: verify
verify: $(VERIFY_DIRS)

.PHONY: verif-%
verif-%: gen-isabelle-root-% copy-makefile-%
	@echo "Verifying Isabelle session in $*..."
	cd $* && make all

.PHONY: copy-makefile-%
copy-makefile-%:
	@rm -f $*/Makefile
	@echo "# This file was automatically generated - modify ../Makefile.template instead" > $*/Makefile
	@cat Makefile.template >> $*/Makefile

.PHONY: gen-isabelle-root-%
gen-isabelle-root-%:
	@rm -f $*/ROOT
	@echo "# This file was automatically generated - see ../Makefile" > $*/ROOT
	@# Use the directory name as the Isabelle session name.
	@# Note: Isabelle session names should ideally be capitalized (e.g., Hello_World),
	@# but using the directory name directly (e.g., hello_world) is simpler and works.
	@echo "session \"$*\"" >> $*/ROOT
	@cat ROOT.template >> $*/ROOT
	@# Find all .thy files *in that directory only* and append their basenames to the ROOT file
	@find $* -maxdepth 1 -name "*.thy" -exec basename {} .thy \; | sed 's/^/  /g' >> $*/ROOT

.PHONY: clean
clean: $(CLEAN_DIRS)

.PHONY: clean-%
clean-%:
	@echo "Cleaning $*..."
	cd $* && make clean