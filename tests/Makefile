.PHONY: all
all: verify-fstar verify-coq verify-lean verify-hol4 verify-isabelle

.PHONY: verify-%
verify-%:
	cd $* && $(MAKE) all
