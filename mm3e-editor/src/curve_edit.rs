//! Atomic edits to one named facial or morph curve in an existing clip.
//! Replacement preserves the track's position; insertion appends. Other clip data,
//! source geometry and authored keys are retained without reconstruction.

use crate::{
    deform::MorphTrack,
    face::{FaceChannel, FaceTrack},
    model::{identifier, Document, Pass},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CurveEdit {
    pub clip: String,
    pub edit: CurveOperation,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum CurveOperation {
    UpsertFace { track: FaceTrack },
    RemoveFace { face: String, channel: FaceChannel },
    UpsertMorph { track: MorphTrack },
    RemoveMorph { deformer: String, blendshape: String },
}

/// Replace, append or remove exactly one curve, committing only a valid document.
/// Missing clips/removal targets and invalid values, key times or references fail
/// without changing the document. Existing animation validation sets all limits.
pub fn apply(document: &mut Document, request: &CurveEdit) -> Result<(), String> {
    identifier(&request.clip)?;
    let index = document
        .clips
        .iter()
        .position(|clip| clip.id == request.clip)
        .ok_or_else(|| format!("missing curve-edit clip {}", request.clip))?;
    let mut candidate = document.clone();
    let clip = &mut candidate.clips[index];
    match &request.edit {
        CurveOperation::UpsertFace { track } => {
            identifier(&track.face)?;
            if let Some(existing) = clip
                .face_tracks
                .iter_mut()
                .find(|existing| existing.face == track.face && existing.channel == track.channel)
            {
                *existing = track.clone();
            } else {
                clip.face_tracks.push(track.clone());
            }
        }
        CurveOperation::RemoveFace { face, channel } => {
            identifier(face)?;
            let index = clip
                .face_tracks
                .iter()
                .position(|track| &track.face == face && &track.channel == channel)
                .ok_or_else(|| format!("missing facial curve {face}/{channel:?}"))?;
            clip.face_tracks.remove(index);
        }
        CurveOperation::UpsertMorph { track } => {
            identifier(&track.deformer)?;
            identifier(&track.blendshape)?;
            if let Some(existing) = clip
                .morph_tracks
                .iter_mut()
                .find(|existing| existing.deformer == track.deformer && existing.blendshape == track.blendshape)
            {
                *existing = track.clone();
            } else {
                clip.morph_tracks.push(track.clone());
            }
        }
        CurveOperation::RemoveMorph { deformer, blendshape } => {
            identifier(deformer)?;
            identifier(blendshape)?;
            let index = clip
                .morph_tracks
                .iter()
                .position(|track| &track.deformer == deformer && &track.blendshape == blendshape)
                .ok_or_else(|| format!("missing morph curve {deformer}/{blendshape}"))?;
            clip.morph_tracks.remove(index);
        }
    }
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}
