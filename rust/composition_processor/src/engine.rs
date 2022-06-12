use dictionary_parser::TableDictionaryEngine;
use itf_components::compartment::Compartment;

use crate::test_virtual_key::{
    test_virtual_key, CandidateMode, KeystrokeCategory, KeystrokeFunction,
};
use windows::{
    core::{AsImpl, GUID},
    Win32::{
        Foundation::{HINSTANCE, MAX_PATH},
        System::LibraryLoader::GetModuleFileNameW,
        UI::{Input::KeyboardAndMouse::VK_SHIFT, TextServices::ITfThreadMgr},
    },
};

pub mod keystroke_buffer;
use keystroke_buffer::KeystrokeBuffer;

mod modifiers;
use modifiers::Modifiers;

mod punctuations;
use punctuations::PunctuationMapper;

mod preserved_keys;
use preserved_keys::PreservedKeys;

pub mod compartment_update_listener;
use compartment_update_listener::CompartmentUpdateListener;

mod language_bar;
use language_bar::LanguageBar;

pub struct CompositionProcessorEngine {
    virtual_key_manager: KeystrokeBuffer,
    table_dictionary_engine: Option<TableDictionaryEngine>,
    modifiers: Modifiers,
    punctuation_mapper: PunctuationMapper,
    preserved_keys: PreservedKeys,
    compartment_wrapper: CompartmentUpdateListener,
    language_bar: LanguageBar,
}

impl CompositionProcessorEngine {
    pub fn new(thread_mgr: ITfThreadMgr, tf_client_id: u32) -> CompositionProcessorEngine {
        CompositionProcessorEngine {
            virtual_key_manager: KeystrokeBuffer::new(),
            table_dictionary_engine: None,
            modifiers: Modifiers::default(),
            punctuation_mapper: PunctuationMapper::new(),
            preserved_keys: PreservedKeys::new(),
            compartment_wrapper: CompartmentUpdateListener::new(thread_mgr, tf_client_id),
            language_bar: LanguageBar::new(),
        }
    }

    pub unsafe fn from_void(engine: *mut std::ffi::c_void) -> Box<CompositionProcessorEngine> {
        Box::from_raw(engine as *mut CompositionProcessorEngine)
    }

    pub fn setup_language_profile(
        &mut self,
        // langid: u16,
        // language_profile: &GUID,
        thread_mgr: ITfThreadMgr,
        client_id: u32,
    ) -> bool {
        if client_id == 0 {
            return false;
        }

        // TODO: fields?

        self.preserved_keys()
            .init_keys(thread_mgr.clone(), client_id)
            .ok();
        self.compartment_wrapper
            .init(thread_mgr.clone(), client_id)
            .ok();
        self.language_bar
            .init(thread_mgr, client_id, &self.compartment_wrapper)
            .ok();
        unsafe { ime::font::set_default_candidate_text_font() };
        self.setup_dictionary_file(
            unsafe { ime::dll::DLL_INSTANCE },
            ime::resources::TEXTSERVICE_DIC,
        );

        true
    }

    pub fn test_virtual_key(
        &self,
        code: u16,
        ch: char,
        composing: bool,
        candidate_mode: CandidateMode,
    ) -> (bool, KeystrokeCategory, KeystrokeFunction) {
        test_virtual_key(self, code, ch, composing, candidate_mode)
    }

    pub fn on_preserved_key(
        &self,
        guid: &GUID,
        thread_mgr: ITfThreadMgr,
        client_id: u32,
    ) -> windows::core::Result<bool> {
        let matching = self
            .preserved_keys
            .keys
            .iter()
            .find(|&preserved| preserved.key_guid == *guid);
        if matching.is_none() {
            return Ok(false);
        }

        let preserved = matching.unwrap();

        if preserved.key.uVKey == VK_SHIFT.0 as u32 && !self.modifiers.is_shift_key_down_only() {
            return Ok(false);
        }

        let compartment = Compartment::new(thread_mgr, client_id, preserved.compartment_guid);
        let state = compartment.get_bool()?;
        compartment.set_bool(!state)?;

        Ok(true)
    }

    fn setup_dictionary_file(
        &mut self,
        dll_instance_handle: HINSTANCE,
        dictionary_file_name: &str,
    ) {
        let file_name = unsafe {
            let mut file_name = [0u16; MAX_PATH as usize];
            GetModuleFileNameW(dll_instance_handle, &mut file_name);
            String::from_utf16(&file_name).unwrap()
        };

        let dir = std::path::Path::new(&file_name[..]).parent().unwrap();
        let dict_path = dir.join(dictionary_file_name);

        self.table_dictionary_engine =
            Some(TableDictionaryEngine::load(dict_path.to_str().unwrap()).unwrap())
    }

    pub fn set_language_bar_status(&mut self, status: u32, set: bool) -> windows::core::Result<()> {
        self.language_bar.button().as_impl().set_status(status, set)
    }

    pub fn get_table_dictionary_engine(&self) -> &Option<TableDictionaryEngine> {
        &self.table_dictionary_engine
    }

    pub fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn modifiers_mut(&mut self) -> &mut Modifiers {
        &mut self.modifiers
    }

    pub fn punctuation_mapper_mut(&mut self) -> &mut PunctuationMapper {
        &mut self.punctuation_mapper
    }

    pub fn preserved_keys(&self) -> &PreservedKeys {
        &self.preserved_keys
    }

    pub fn keystroke_buffer(&self) -> &KeystrokeBuffer {
        &self.virtual_key_manager
    }

    pub fn keystroke_buffer_mut(&mut self) -> &mut KeystrokeBuffer {
        &mut self.virtual_key_manager
    }

    pub fn compartment_wrapper(&self) -> &CompartmentUpdateListener {
        &self.compartment_wrapper
    }
}
