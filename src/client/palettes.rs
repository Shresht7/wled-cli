use crate::api::{endpoints::Endpoint, pal::PaletteList};

impl super::WLEDClient {
    /// Get a list of all available palettes
    pub fn get_palettes(&self) -> Result<PaletteList, Box<dyn std::error::Error>> {
        let url = Endpoint::Pal.url(&self.host);
        let response = self.client.get(url).send()?.json()?;
        Ok(response)
    }
}
