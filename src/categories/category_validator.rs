use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::attributes::attribute_id_tracker::{AttributeEntry, AttributeIdTracker};
use crate::attributes::attribute_id_tracker_file_io::AttributeIdTrackerFileIO;
use crate::attributes::attribute_id_tracker_io::AttributeIdTrackerIO;
use crate::attributes::attribute_validator::AttributeValidator;
use crate::attributes::attribute_validator_builder::build_attribute_validator;
use crate::categories::category_id_tracker_file_io::CategoryIdTrackerFileIO;

use super::category_file_io::build_for_all_categories;
use super::category_id_tracker::{CategoryEntry, CategoryIdTracker};
use super::category_id_tracker_io::CategoryIdTrackerIO;
use super::category_io::CategoryIO;

pub struct CategoryValidator {}

impl CategoryValidator {
    fn new() -> CategoryValidator {
        CategoryValidator {}
    }
}