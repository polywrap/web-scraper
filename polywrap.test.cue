import "github.com/polywrap/standard-library/cue/http/v0.0.1/module.cue"

package wrap

get_links: http.v0_0_1.ArgsGetLinks & {
	uri: "https://polywrap.io"
}

get_text: http.v0_0_1.ArgsGet & {
	url: "https://polywrap.io"
}
