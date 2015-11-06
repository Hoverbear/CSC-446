# $< is input.
# $@ is output.

GRAPHS = $(wildcard graphs/*.dot)
IMAGES = $(patsubst graphs/%.dot,img/%.png,$(GRAPHS))

all: $(IMAGES) answers.pdf

img/%.png: graphs/%.dot
	mkdir -p img
	dot -Tpng $< -o $@

answers.pdf: answers.md
	pandoc --smart --standalone --highlight-style=haddock -f markdown -t latex -o $@ $<

clean:
	rm -rf img
	rm answers.pdf
