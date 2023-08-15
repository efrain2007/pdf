use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::Size;
use genpdf::{elements, fonts, style};

const FONT_DIRS: &[&str] = &[r"C:\archivos\font", r"C:\archivos\font"];
const DEFAULT_FONT_NAME: &'static str = "Roboto";

fn main() {
    let expected_dir = std::path::Path::new("files/hola.pdf");
    if !expected_dir.exists() {
        std::fs::create_dir(&expected_dir).expect("Failed to create expected directory");
    }
    let output_file = expected_dir;

    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font = fonts::from_files(font_dir, DEFAULT_FONT_NAME, None)
        .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(1);
    doc.set_page_decorator(decorator);
    doc.set_paper_size(Size::new(70, 400));
    doc.set_title("Factura");
    doc.set_minimal_conformance();

    //doc.set_line_spacing(1.5);

    doc.push(
        elements::Paragraph::new("Factura  F005-000055252")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(20)),
    );

    // Información del emisor
    doc.push(elements::Paragraph::new("MiEmpr").aligned(Alignment::Left));
    doc.push(elements::Paragraph::new("Calle 123, ").aligned(Alignment::Left));
    doc.push(elements::Paragraph::new("Tel: +1").aligned(Alignment::Left));

    // Crear tabla de productos
    let mut table = elements::TableLayout::new(vec![3.0, 0.8, 1.5]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, true));

    // Cabeceras
    table
        .row()
        .element(elements::Paragraph::new("Productos").padded(1))
        .element(elements::Paragraph::new("Cantidad").padded(1))
        .element(elements::Paragraph::new("SubTotal").padded(0))
        .push()
        .expect("Invalid table header row");

    // Producto 1
    table
        .row()
        .element(elements::Paragraph::new(r"Everyone has the right to freedom of thought, conscience  and religion; this right includes freedom to change his religion or belief, and freedom, either alone or in community with others and in public or private, to manifest his religion or belief in teaching,  practice, worship and observance. Everyone has the right to freedom of opinion and expression; this  right includes freedom to hold opinions without interference and to seek, receive and impart information and ideas through any media and regardless of frontiers. Everyone has the right to rest and leisure, including reasonable limitation of working hours and periodic holidays   with pay.").styled(style::Style::new().bold().with_font_size(8)).padded(1))
        .element(elements::Paragraph::new("2").styled(style::Style::new().bold().with_font_size(20),).padded(1))
        .element(elements::Paragraph::new("10000.00").aligned(Alignment::Right).padded(1))
        .push()
        .expect("Invalid table row for Producto 1");

    //Producto 2
    table
        .row()
        .element(elements::Paragraph::new("Producto 2").padded(1))
        .element(elements::Paragraph::new("5").padded(1))
        .element(elements::Paragraph::new("$50.00").padded(1))
        .push()
        .expect("Invalid table row for Producto 2");

    doc.push(table);

    // Total
    let total = 2.0 * 150.0 + 5.0 * 50.0;
    doc.push(
        elements::Paragraph::new(&format!("Total: ${:.2}", total))
            .aligned(Alignment::Right)
            .styled(style::Style::new().bold()),
    );

    doc.render_to_file(output_file)
        .expect("Failed to write output file");
}

// // SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// // SPDX-License-Identifier: CC0-1.0

// //! This example generates a demo PDF document and writes it to the path that was passed as the
// //! first command-line argument.  You may have to adapt the `FONT_DIRS`, `DEFAULT_FONT_NAME` and
// //! `MONO_FONT_NAME` constants for your system so that these files exist:
// //! - `{FONT_DIR}/{name}-Regular.ttf`
// //! - `{FONT_DIR}/{name}-Bold.ttf`
// //! - `{FONT_DIR}/{name}-Italic.ttf`
// //! - `{FONT_DIR}/{name}-BoldItalic.ttf`
// //! for `name` in {`DEFAULT_FONT_NAME`, `MONO_FONT_NAME`}.
// //!
// //! The generated document using the latest `genpdf-rs` release is available
// //! [here](https://genpdf-rs.ireas.org/examples/demo.pdf).

// use genpdf::Alignment;
// use genpdf::Element as _;
// use genpdf::{elements, fonts, style};

// const FONT_DIRS: &[&str] = &[
//     r"C:\archivos\font",
//     r"C:\archivos\font",
// ];
// const DEFAULT_FONT_NAME: &'static str = "LiberationSans";
// const MONO_FONT_NAME: &'static str = "LiberationSans";
// const LOREM_IPSUM: &'static str =
//     "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
//     labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco \
//     laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
//     voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat \
//     non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

// fn main() {
//     let expected_dir = std::path::Path::new("files/hola.pdf");
//     if !expected_dir.exists() {
//         std::fs::create_dir(&expected_dir).expect("Failed to create expected directory");
//     }
//     let output_file = expected_dir;

//     let font_dir = FONT_DIRS
//         .iter()
//         .filter(|path| std::path::Path::new(path).exists())
//         .next()
//         .expect("Could not find font directory");
//     let default_font =
//         fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
//             .expect("Failed to load the default font family");
//     let monospace_font = fonts::from_files(font_dir, MONO_FONT_NAME, Some(fonts::Builtin::Courier))
//         .expect("Failed to load the monospace font family");

//     let mut doc = genpdf::Document::new(default_font);
//     doc.set_title("genpdf Demo Document");
//     doc.set_minimal_conformance();
//     doc.set_line_spacing(1.25);

//     let mut decorator = genpdf::SimplePageDecorator::new();
//     decorator.set_margins(10);
//     decorator.set_header(|page| {
//         let mut layout = elements::LinearLayout::vertical();
//         if page > 1 {
//             layout.push(
//                 elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Center),
//             );
//             layout.push(elements::Break::new(1));
//         }
//         layout.styled(style::Style::new().with_font_size(10))
//     });
//     doc.set_page_decorator(decorator);

//     #[cfg(feature = "hyphenation")]
//     {
//         use hyphenation::Load;

//         doc.set_hyphenator(
//             hyphenation::Standard::from_embedded(hyphenation::Language::EnglishUS)
//                 .expect("Failed to load hyphenation data"),
//         );
//     }

//     let monospace = doc.add_font_family(monospace_font);
//     let code = style::Style::from(monospace).bold();
//     let red = style::Color::Rgb(255, 0, 0);
//     let blue = style::Color::Rgb(0, 0, 255);

//     doc.push(
//         elements::Paragraph::new("genpdf Demo Document")
//             .aligned(Alignment::Center)
//             .styled(style::Style::new().bold().with_font_size(20)),
//     );
//     doc.push(elements::Break::new(1.5));
//     doc.push(elements::Paragraph::new(
//         "This document demonstrates how the genpdf crate generates PDF documents. Currently, \
//          genpdf supports these elements:",
//     ));

//     let mut list = elements::UnorderedList::new();
//     list.push(
//         elements::Paragraph::default()
//             .styled_string("Text", code)
//             .string(", a single line of formatted text without wrapping."),
//     );
//     list.push(
//         elements::Paragraph::default()
//             .styled_string("Paragraph", code)
//             .string(
//                 ", one or more lines of formatted text with wrapping and an alignment (left, \
//                  center, right).",
//             ),
//     );
//     list.push(
//         elements::Paragraph::default()
//             .styled_string("FramedElement", code)
//             .string(", a frame drawn around other elements."),
//     );
//     list.push(
//         elements::Paragraph::default()
//             .styled_string("PaddedElement", code)
//             .string(", an element with an additional padding."),
//     );
//     list.push(
//         elements::Paragraph::default()
//             .styled_string("StyledElement", code)
//             .string(", an element with new default style."),
//     );

//     list.push(
//         elements::Paragraph::default()
//             .styled_string("UnorderedList", code)
//             .string(", an unordered list of bullet points."),
//     );

//     list.push(
//         elements::LinearLayout::vertical()
//             .element(
//                 elements::Paragraph::default()
//                     .styled_string("OrderedList", code)
//                     .string(", an ordered list of bullet points."),
//             )
//             .element(
//                 elements::OrderedList::new()
//                     .element(elements::Paragraph::new("Just like this."))
//                     .element(elements::Paragraph::new("And this.")),
//             ),
//     );

//     list.push(
//         elements::LinearLayout::vertical()
//             .element(
//                 elements::Paragraph::default()
//                     .styled_string("BulletPoint", code)
//                     .string(", an element with a bullet point, just like in this list."),
//             )
//             .element(elements::BulletPoint::new(elements::Paragraph::new(
//                 "Of course, lists can also be nested.",
//             )))
//             .element(
//                 elements::BulletPoint::new(elements::Paragraph::new(
//                     "And you can change the bullet symbol.",
//                 ))
//                 .with_bullet("•"),
//             ),
//     );

//     list.push(
//         elements::Paragraph::default()
//             .styled_string("LinearLayout", code)
//             .string(
//                 ", a container that vertically stacks its elements. The root element of a \
//                  document is always a LinearLayout.",
//             ),
//     );
//     list.push(
//         elements::Paragraph::default()
//             .styled_string("TableLayout", code)
//             .string(", a container that arranges its elements in rows and columns."),
//     );
//     list.push(elements::Paragraph::new("And some more utility elements …"));
//     doc.push(list);
//     doc.push(elements::Break::new(1.5));

//     doc.push(elements::Paragraph::new(
//         "You already saw lists and formatted centered text. Here are some other examples:",
//     ));
//     doc.push(elements::Paragraph::new("This is right-aligned text.").aligned(Alignment::Right));
//     doc.push(
//         elements::Paragraph::new("And this paragraph has a frame drawn around it and is colored.")
//             .padded(genpdf::Margins::vh(0, 1))
//             .framed(style::LineStyle::new())
//             .styled(red),
//     );
//     doc.push(
//         elements::Paragraph::new("You can also use other fonts if you want to.").styled(monospace),
//     );
//     doc.push(
//         elements::Paragraph::default()
//             .string("You can also ")
//             .styled_string("combine ", red)
//             .styled_string("multiple ", style::Style::from(blue).italic())
//             .styled_string("formats", code)
//             .string(" in one paragraph.")
//             .styled(style::Style::new().with_font_size(16)),
//     );
//     doc.push(elements::Break::new(1.5));

//     doc.push(elements::Paragraph::new(
//         "Embedding images also works using the 'images' feature.",
//     ));
//     #[cfg(feature = "images")]
//     images::do_image_test(&mut doc);

//     doc.push(elements::Paragraph::new("Here is an example table:"));

//     let mut table = elements::TableLayout::new(vec![1, 2]);
//     table.set_cell_decorator(elements::FrameCellDecorator::new(true, false, false));
//     table
//         .row()
//         .element(
//             elements::Paragraph::new("Header 1")
//                 .styled(style::Effect::Bold)
//                 .padded(1),
//         )
//         .element(elements::Paragraph::new("Value 2").padded(1))
//         .push()
//         .expect("Invalid table row");
//     table
//         .row()
//         .element(
//             elements::Paragraph::new("Header 2")
//                 .styled(style::Effect::Bold)
//                 .padded(1),
//         )
//         .element(
//             elements::Paragraph::new(
//                 "A long paragraph to demonstrate how wrapping works in tables.  Nice, right?",
//             )
//             .padded(1),
//         )
//         .push()
//         .expect("Invalid table row");
//     let list_layout = elements::LinearLayout::vertical()
//         .element(elements::Paragraph::new(
//             "Of course, you can use all other elements inside a table.",
//         ))
//         .element(
//             elements::UnorderedList::new()
//                 .element(elements::Paragraph::new("Even lists!"))
//                 .element(
//                     elements::Paragraph::new("And frames!")
//                         .padded(genpdf::Margins::vh(0, 1))
//                         .framed(style::LineStyle::new()),
//                 ),
//         );
//     table
//         .row()
//         .element(
//             elements::Paragraph::new("Header 3")
//                 .styled(style::Effect::Bold)
//                 .padded(1),
//         )
//         .element(list_layout.padded(1))
//         .push()
//         .expect("Invalid table row");
//     doc.push(table);
//     doc.push(elements::Break::new(1.5));

//     doc.push(elements::Paragraph::new(
//         "Now let’s print a long table to demonstrate how page wrapping works:",
//     ));

//     let mut table = elements::TableLayout::new(vec![1, 5]);
//     table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
//     table
//         .row()
//         .element(
//             elements::Paragraph::new("Index")
//                 .styled(style::Effect::Bold)
//                 .padded(1),
//         )
//         .element(
//             elements::Paragraph::new("Text")
//                 .styled(style::Effect::Bold)
//                 .padded(1),
//         )
//         .push()
//         .expect("Invalid table row");
//     for i in 0..10 {
//         table
//             .row()
//             .element(elements::Paragraph::new(format!("#{}", i)).padded(1))
//             .element(elements::Paragraph::new(LOREM_IPSUM).padded(1))
//             .push()
//             .expect("Invalid table row");
//     }

//     doc.push(table);

//     doc.render_to_file(output_file)
//         .expect("Failed to write output file");
// }
