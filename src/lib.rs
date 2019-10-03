// Copyright (c) 2018-2019  Ministerio de Fomento
//                          Instituto de Ciencias de la Construcción Eduardo Torroja (IETcc-CSIC)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Author(s): Rafael Villar Burke <pachi@ietcc.csic.es>,
//            Daniel Jiménez González <dani@ietcc.csic.es>,
//            Marta Sorribes Gil <msorribes@ietcc.csic.es>

/*!
CteEPBD
=======

This crate provides a library and binary that **implements most of the ISO EN 52000-1 standard**
*Energy performance of buildings - Overarching EPB assessment - General framework and procedures*
(under version EN ISO 52000-1:2017).

This is oriented towards the assessment of the energy performance of buildings under the
spanish building code (CTE) and, thus, uses specific naming conventions and default values
best suited for that purpose.

It also holds the following assumptions:

- constant weighting factors through all timesteps
- no priority is defined for energy production (average step A weighting factor f_we_el_stepA)
- all on-site produced energy from non cogeneration sources is considered as delivered
- on-site produced energy is not compensated on a service by service basis, but on a carrier basis
- unit and constant load matching factor

Some restrictions may be lifted in the future. Specifically:

- implement a load matching factor (f_match_t) following formula B.32 in appendix B
- allow the imputation to a specific service for produced energy
- allow setting priorities for energy production

Este *crate* proporciona una biblioteca y un programa que **implementa una parte sustancial del
estándar EN ISO 52000-1**: *Eficiencia energética de los edificios - Evaluación global de la EPB - 
Parte 1: Marco general y procedimientos* (versión EN ISO 52000-1:2017).

Este software está orientado a la evaluación de la eficiencia energética de los edificios dentro
del marco de la normativa española de edificación (Código Técnico de la Edificación CTE, DB-HE) y,
así, adopta nomenclatura y valores por defecto adaptados a ese propósito.

También realiza los siguientes supuestos:

- factores de paso constantes en todo el periodo de cálculo
- no se definen prioridades para la producción de energía
- se considera como suministrada toda la energía producida procedente de fuentes distintas a la cogeneración
- la energía producida in situ se compensa por vector energético y no por servicios
- factor de coincidencia de cargas igual a la unidad

Algunas restricciones pueden revisarse en el futuro, tales como:

- implementación del factor de coincidencia de cargas según fórmula B.32 del apéndice B
- imputación de energía generada a servicios específicos
- fijación de prioridades para la producción de energía

*/

#![deny(missing_docs)]

#[cfg(test)] // <-- not needed in examples + integration tests
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate serde_derive;

mod balance;
mod components;
pub mod cte;
pub mod error;
pub mod types;
mod vecops;
mod wfactors;

pub use balance::*;
pub use components::*;
pub use wfactors::*;

/// Número de versión de la librería
/// 
/// Version number
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
