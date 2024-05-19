mod testing;

mod command {
    mod input;
    mod output;
    mod template;
}
mod markdown {
    mod matter {
        mod kinds {
            mod html;
            mod json;
            mod markdown_comments;
            mod toml;
            mod yaml;
        }
    }
}
mod render {
    mod minimal;
    mod title;
    mod matter_with_html;
    mod matter_with_json;
    mod matter_with_markdown_comments;
    mod matter_with_toml;
    mod matter_with_yaml;
}
// mod src {
//     mod f {
//         mod from_pathable_string_into_list_path_buf;
//         mod from_pathable_string_into_list_path_buf/a;
//         mod from_pathable_string_into_list_path_buf/a/aa;
//         mod from_pathable_string_into_list_path_buf/a/ab;
//         mod from_pathable_string_into_list_path_buf/b;
//         mod from_pathable_string_into_list_path_buf/b/ba;
//         mod from_pathable_string_into_list_path_buf/b/bb;
//         mod from_pathable_string_into_list_path_buf/c;
//         mod from_pathable_string_into_list_path_buf/c/ca;
//         mod from_pathable_string_into_list_path_buf/c/cb;
//         mod from_pathable_string_into_list_path_buf/d;
//         mod from_pathable_string_into_list_path_buf/d/da;
//         mod from_pathable_string_into_list_path_buf/d/db;
//         mod from_set_pathable_string_into_set_path_buf;
//         mod from_set_pathable_string_into_set_path_buf/a;
//         mod from_set_pathable_string_into_set_path_buf/a/aa;
//         mod from_set_pathable_string_into_set_path_buf/a/ab;
//         mod from_set_pathable_string_into_set_path_buf/b;
//         mod from_set_pathable_string_into_set_path_buf/b/ba;
//         mod from_set_pathable_string_into_set_path_buf/b/bb;
//         mod from_set_pathable_string_into_set_path_buf/c;
//         mod from_set_pathable_string_into_set_path_buf/c/ca;
//         mod from_set_pathable_string_into_set_path_buf/c/cb;
//         mod from_set_pathable_string_into_set_path_buf/d;
//         mod from_set_pathable_string_into_set_path_buf/d/da;
//         mod from_set_pathable_string_into_set_path_buf/d/db;
//         mod read_content_as_mix_text;
//         mod vet_input_file_path_buf_exists;
//         mod vet_input_file_path_buf_metadata;
//     }
// }
// mod templater {
//     mod templater_trait {
// ./src/templater/templater_trait/register_helper_via_name_and_content_file
// ./src/templater/templater_trait/register_template_via_name_and_content_file
// ./src/templater/templater_with_handlebars
// ./src/templater/templater_with_handlebars/register_helper_via_name_and_content_file
// ./src/templater/templater_with_handlebars/register_template_via_name_and_content_file
// ./src/templater/templater_with_tera
// ./src/templater/templater_with_tera/register_helper_via_name_and_content_file
// ./src/templater/templater_with_tera/register_template_via_name_and_content_file
mod tutorial {
    mod tutorial_01_input;
    mod tutorial_02_output;
    mod tutorial_03_template;
}
