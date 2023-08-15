// SPDX-FileCopyrightText: 2020-2021 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Utilities for text wrapping.

use std::mem;

use crate::style;
use crate::Context;
use crate::Mm;

/// Combines a sequence of styled words into lines with a maximum width.
///
/// If a word does not fit into a line, the wrapper tries to split it using the `split` function.
pub struct Wrapper<'c, 's, I: Iterator<Item = style::StyledStr<'s>>> {
    iter: I,
    context: &'c Context,
    width: Mm,
    x: Mm,
    buf: Vec<style::StyledCow<'s>>,
    has_overflowed: bool,
}

impl<'c, 's, I: Iterator<Item = style::StyledStr<'s>>> Wrapper<'c, 's, I> {
    /// Creates a new wrapper for the given word sequence and with the given maximum width.
    pub fn new(iter: I, context: &'c Context, width: Mm) -> Wrapper<'c, 's, I> {
        Wrapper {
            iter,
            context,
            width,
            x: Mm(0.0),
            buf: Vec::new(),
            has_overflowed: false,
        }
    }

    /// Returns true if this wrapper has overflowed, i. e. if it encountered a word that it could
    /// not split so that it would fit into a line.
    pub fn has_overflowed(&self) -> bool {
        self.has_overflowed
    }
}

impl<'c, 's, I: Iterator<Item = style::StyledStr<'s>>> Iterator for Wrapper<'c, 's, I> {
    // This iterator yields pairs of lines and the length difference between the input words and
    // the line.
    type Item = (Vec<style::StyledCow<'s>>, usize);

    fn next(&mut self) -> Option<(Vec<style::StyledCow<'s>>, usize)> {
        // Agregue palabras a self.buf hasta que se alcance la longitud máxima de línea
        while let Some(s) = self.iter.next() {
            // println!("{:?}",s);
            let mut width = s.width(&self.context.font_cache);
            // println!("primer {:?}",width);
            if self.x + width > self.width {
                // La palabra no cabe en la línea actual (al menos no completamente)

                let mut delta = 0;
                // Intente dividir la palabra para que la primera parte quepa en la línea actual
                let s = if let Some((start, end)) = split(self.context, s, self.width - self.x) {
                    // Calcula la cantidad de bytes que agregamos a la cadena al dividirla
                    // (para el guión, si es necesario).
                    delta = start.s.len() + end.s.len() - s.s.len();
                    self.buf.push(start);
                    width = end.width(&self.context.font_cache);
                    println!("que fue{:}", s.s);
                    end
                } else {
                    println!("salto{:}", s.s);
                    s.into()
                };

                if width > self.width {
                    //println!("{:?}",width);
                    println!("entro en qiw{:?}", self.width);
                    println!("paso wit{:}", s.s);
                    // El resto de la palabra es más largo que la página actual: nunca seremos
                    // capaz de hacerlo completamente.
                    // TODO: manejar con gracia, emitir advertencia
                    self.has_overflowed = true;
                    // return None; se esta plainifacando aun
                }

                // Devuelve la línea actual y agrega la palabra que no encajaba en la línea siguiente
                let v = std::mem::take(&mut self.buf);
                println!("guadarndo{:?}", s.s);
                self.buf.push(s);
                self.x = width;
                return Some((v, delta));
            } else {
                // La palabra cabe en la línea actual, así que simplemente agréguela
                self.buf.push(s.into());
                self.x += width;
            }
        }

        if self.buf.is_empty() {
            None
        } else {
            Some((mem::take(&mut self.buf), 0))
        }
    }
}

#[cfg(not(feature = "hyphenation"))]
fn split<'s>(
    context: &Context,
    s: style::StyledStr<'s>,
    len: Mm,
) -> Option<(style::StyledCow<'s>, style::StyledCow<'s>)> {
    //     println!("---len---{:?}", len);
    //     let mut current_len = Mm(0.0);
    //     let mut last_split_idx = None;
    //     println!("------{:}", s.s);
    //    // for (idx, ch) in s.s.char_indices() {

    //         current_len += s.width(&context.font_cache);
    //         println!("---current_len---{:?}", current_len);// Suponiendo que 'width' es una función válida para caracteres
    //         if current_len > len {
    //           //  break;
    //         }
    //         println!("---s.s.len()---{:?}", s.s.len());
    //         let cure =  current_len.0  / 2.5;
    //         println!("---cure---{:?}", cure);
    //         let ssss = len.0 - cure   ;
    //         println!("---ssss---{:?}", ssss);

    //        // if ch.is_whitespace() || ch == '-' {
    //             last_split_idx = Some(ssss.abs() as usize);

    //       //  }
    //    // }
    //    println!("---last_split_idx---{:?}", last_split_idx);
    //     if let Some(split_idx) = last_split_idx {
    //         let (start, end) = s.s.split_at(split_idx);
    //         Some((
    //             style::StyledCow::new(start, s.style),
    //             style::StyledCow::new(end, s.style),

    //         ))
    //     } else {
    //         None
    //     }

    let mark = "";
    let mark_width = s.style.str_width(&context.font_cache, mark);

    let hyphenated = s.s;
    let segments: Vec<_> = hyphenated.chars().collect();

    // aqui se saca su indice
    let idx = segments
        .iter()
        .scan(Mm(0.0), |acc, t| {
            *acc += s.style.str_width(&context.font_cache, &t.to_string());
            Some(*acc)
        })
        .position(|w| w + mark_width > len)
        .unwrap_or_default();
    if idx > 0 {
        // let idx = hyphenated.char_indices().count();
        let start = s.s[..idx].to_owned() + mark;
        let end = &s.s[idx..];
        Some((
            style::StyledCow::new(start, s.style),
            style::StyledCow::new(end, s.style),
        ))
    } else {
        None
    }
}

/// Tries to split the given string into two parts so that the first part is shorter than the given
/// width.
#[cfg(feature = "hyphenation")]
fn split<'s>(
    context: &Context,
    s: style::StyledStr<'s>,
    width: Mm,
) -> Option<(style::StyledCow<'s>, style::StyledCow<'s>)> {
    use hyphenation::{Hyphenator, Iter};

    let hyphenator = if let Some(hyphenator) = &context.hyphenator {
        hyphenator
    } else {
        return None;
    };

    let mark = "-";
    let mark_width = s.style.str_width(&context.font_cache, mark);

    let hyphenated = hyphenator.hyphenate(s.s);
    let segments: Vec<_> = hyphenated.iter().segments().collect();

    // Find the hyphenation with the longest first part so that the first part (and the hyphen) are
    // shorter than or equals to the required width.
    let idx = segments
        .iter()
        .scan(Mm(0.0), |acc, t| {
            *acc += s.style.str_width(&context.font_cache, t);
            Some(*acc)
        })
        .position(|w| w + mark_width > width)
        .unwrap_or_default();
    if idx > 0 {
        let idx = hyphenated.breaks[idx - 1];
        let start = s.s[..idx].to_owned() + mark;
        let end = &s.s[idx..];
        Some((
            style::StyledCow::new(start, s.style),
            style::StyledCow::new(end, s.style),
        ))
    } else {
        None
    }
}

/// Splits a sequence of styled strings into words.
pub struct Words<I: Iterator<Item = style::StyledString>> {
    iter: I,
    s: Option<style::StyledString>,
}

impl<I: Iterator<Item = style::StyledString>> Words<I> {
    /// Creates a new words iterator.
    pub fn new<IntoIter: IntoIterator<Item = style::StyledString, IntoIter = I>>(
        iter: IntoIter,
    ) -> Words<I> {
        Words {
            iter: iter.into_iter(),
            s: None,
        }
    }
}

impl<I: Iterator<Item = style::StyledString>> Iterator for Words<I> {
    type Item = style::StyledString;

    fn next(&mut self) -> Option<style::StyledString> {
        if self.s.as_ref().map(|s| s.s.is_empty()).unwrap_or(true) {
            self.s = self.iter.next();
        }

        if let Some(s) = &mut self.s {
            // Divida en el primer espacio o use la cadena completa
            let n = s.s.find(' ').map(|i| i + 1).unwrap_or_else(|| s.s.len());
            let mut tmp = s.s.split_off(n);

            mem::swap(&mut tmp, &mut s.s);

            Some(style::StyledString::new(tmp, s.style))
        } else {
            None
        }
    }
}
