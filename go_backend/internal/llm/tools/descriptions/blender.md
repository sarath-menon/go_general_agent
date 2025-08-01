# Video editing tools
These are python tools for video editing automation using the blender MCP 

## Instructions

- Use ONLY with `execute_blender_code` - these are Blender-specific Python function
- Blender has a "Video Editing" workspace, even though it may not be listed when you check with bpy
- You MUST run the final python script within blender's execution environment as a SINGLE code block. Don't create separate files or split into multiple execution calls - each `execute_blender_code` call is stateless and variables/imports don't persist between calls.
-  You MUST use absolute file paths when working, since Blender's working directory may not match your project directory.
- **ALL functions are exposed through `blender` - NEVER import from submodules:**
    **Positive example**

    ```python
    import sys, json
    tools_path = filepath.Join($<workdir>, "go_backend", "tools")
    sys.path.insert(0, tools_path)
    from blender import (
        get_timeline_items
    )
    ```

## ⚠️ Sequencer Rules

- **No overlapping video/audio/image clips**  - ALWAYS use **a different channel** for each one.
    - each one starts where previous ends
    - Calculate frame positions: `next_start = previous_start + previous_duration`
    - Example: Video1 ch1 (1-100), Video2 ch2 (101-200), Video3 ch1 (201-300) ✓

**Common Patterns:**
```python
# Sequential: clips play one after another
add_video('clip1.mp4', 1, 1)      # frames 1-120
add_video('clip2.mp4', 1, 121)    # frames 121-240  
add_video('clip3.mp4', 1, 241)    # frames 241-360

# Layered: clips play simultaneously 
add_video('background.mp4', 1, 1)  # base layer
add_video('overlay.mp4', 2, 50)    # on top, starts at frame 50
add_text('Title', 3, 100, 60)      # text overlay
```

## Quick Start Workflow

```python
# Check available workspaces and switch to Video Editing
current = get_current_workspace()
print(f"Current workspace: {current}")

# Set timeline frame range
set_frame_range(1, 300)
frame_info = get_frame_range()
print(f"Timeline: {frame_info['frame_start']}-{frame_info['frame_end']}")

# Add background color and media to timeline
add_color((0.1, 0.1, 0.1, 1.0), 1, 0, 300)
add_video('/path/to/video.mp4', 2, 10)
add_text('Hello World', 3, 50, 120, font_size=72)

# Add transition between sequences
add_transition('video1', 'video2', 'CROSS', 15)

# Cut a video sequence at frame 100 (creates two separate sequences)
blade_cut('my_video', 100, 'video_part1', 'video_part2')

# Set playhead position and export
set_current_frame(150)
export_video('/path/to/output.mp4', resolution=(1920, 1080), fps=24)

# Capture preview screenshot at current frame
capture_preview_frame('/path/to/preview.png', resolution=(1920, 1080))

# Capture specific frame as JPEG
capture_preview_frame('/path/to/frame_100.jpg', frame=100, format='JPEG', quality=85)
```


### get_current_workspace()
Returns the name of the currently active Blender workspace

**Returns:**
- `str` - The workspace name (e.g., "Video Editing", "Modeling")

### get_timeline_items(channel=None)
Returns timeline sequences, optionally filtered by channel

**Parameters:**
- `channel` (Optional[int]) - Channel number to filter by, None for all sequences

**Returns:**
- `List[Dict[str, Any]]` - List of sequence dictionaries with name, type, channel, frame_start, frame_end, duration, filepath, original_resolution, transform, is_resized

### delete_timeline_item((sequence_name)
Delete a sequence from the timeline by name

**Parameters:**
- `sequence_name` (str) - Name of the sequence to delete

**Returns:**
- `Dict[str, Any]` - Dictionary with success status, message, and deleted sequence info

### blade_cut(sequence_name, cut_frame, left_name=None, right_name=None)
Cut a video or audio sequence at a specific frame, creating two separate sequences

**Parameters:**
- `sequence_name` (str) - Name of the sequence to cut (must be MOVIE or SOUND type)
- `cut_frame` (int) - Frame position where to make the cut
- `left_name` (Optional[str]) - Name for the left part (auto-generated if None)
- `right_name` (Optional[str]) - Name for the right part (auto-generated if None)

**Returns:**
- `Dict[str, Any]` - Dictionary with success status, message, original sequence info, and info about both created sequences

**Note:** This is the equivalent of a "razor blade" or "blade" tool in video editors. The original sequence is removed and replaced with two new sequences. Only works with MOVIE (video) and SOUND (audio) sequences.

### get_sequence_resize_info(sequence_name)
Get detailed resize information for a specific sequence

**Parameters:**
- `sequence_name` (str) - Name of the sequence to check

**Returns:**
- `Dict[str, Any]` - Detailed resize information with original_resolution, current_scale, effective_resolution, resize_method, fit_method, transform

## Media Operations

### add_image(filepath, channel, frame_start, name=None, fit_method='ORIGINAL', frame_end=None, position=None, scale=None)
Add an image element to the timeline at specific channel and position

**Parameters:**
- `filepath` (str) - Path to image file
- `channel` (int) - Channel number
- `frame_start` (int) - Starting frame position
- `name` (Optional[str]) - Sequence name
- `fit_method` (str) - How to fit image ('ORIGINAL', 'FIT', 'FILL', 'STRETCH')
- `frame_end` (Optional[int]) - Ending frame position (images can be extended to any duration)
- `position` (Optional[Tuple[float, float]]) - (x, y) offset in pixels from center
- `scale` (Optional[float]) - Uniform scale factor (1.0 = original size)

**Returns:**
- `Dict[str, Any]` - Sequence info dictionary matching get_timeline_items() format

**Supported formats:** .jpg, .jpeg, .png, .tiff, .tif, .exr, .hdr, .bmp, .tga

### add_video(filepath, channel, frame_start, name=None, fit_method='ORIGINAL', frame_end=None)
Add a video element to the timeline at specific channel and position

**Parameters:**
- `filepath` (str) - Path to video file
- `channel` (int) - Channel number
- `frame_start` (int) - Starting frame position
- `name` (Optional[str]) - Sequence name
- `fit_method` (str) - How to fit video ('ORIGINAL', 'FIT', 'FILL', 'STRETCH')
- `frame_end` (Optional[int]) - Ending frame position (videos can only be trimmed, not extended)

**Returns:**
- `Dict[str, Any]` - Sequence info dictionary matching get_timeline_items() format

**Supported formats:** .mp4, .mov, .avi, .mkv, .webm, .wmv, .m4v, .flv

### add_audio(filepath, channel, frame_start, name=None, frame_end=None)
Add an audio element to the timeline at specific channel and position

**Parameters:**
- `filepath` (str) - Path to audio file
- `channel` (int) - Channel number
- `frame_start` (int) - Starting frame position
- `name` (Optional[str]) - Sequence name
- `frame_end` (Optional[int]) - Ending frame position (audio can only be trimmed, not extended)

**Returns:**
- `Dict[str, Any]` - Sequence info dictionary matching get_timeline_items() format

**Supported formats:** .wav, .mp3, .flac, .ogg, .aac, .m4a, .wma

### add_text(text, channel, frame_start, duration, name=None, font_size=50, color=(1.0, 1.0, 1.0, 1.0), location=(0, 0), use_background=False, background_color=(0.0, 0.0, 0.0, 0.8))
Add a text element to the timeline at specific channel and position

**Parameters:**
- `text` (str) - The text content to display
- `channel` (int) - Channel number
- `frame_start` (int) - Starting frame position
- `duration` (int) - Duration in frames
- `name` (Optional[str]) - Sequence name
- `font_size` (int) - Font size in pixels
- `color` (Tuple[float, float, float, float]) - Text color as RGBA tuple
- `location` (Tuple[int, int]) - Text position as (X, Y) tuple
- `use_background` (bool) - Whether to show background behind text
- `background_color` (Tuple[float, float, float, float]) - Background color as RGBA tuple

**Returns:**
- `Dict[str, Any]` - Text sequence info dictionary matching get_timeline_items() format

### add_color(color, channel, frame_start, duration, name=None)
Add a solid color element to the timeline at specific channel and position. DO NOT use it to add images.

**Parameters:**
- `color` (Tuple[float, float, float, float]) - Color as RGBA tuple (values 0.0-1.0)
- `channel` (int) - Channel number
- `frame_start` (int) - Starting frame position
- `duration` (int) - Duration in frames
- `name` (Optional[str]) - Sequence name

**Returns:**
- `Dict[str, Any]` - Color sequence info dictionary matching get_timeline_items() format

### add_transition(sequence1_name, sequence2_name, transition_type='CROSS', duration=10, channel=None)
Add a transition effect between two sequences on the timeline

**Parameters:**
- `sequence1_name` (str) - Name of the first sequence
- `sequence2_name` (str) - Name of the second sequence
- `transition_type` (str) - Type of transition ('CROSS', 'WIPE', 'GAMMA_CROSS')
- `duration` (int) - Duration of transition in frames
- `channel` (Optional[int]) - Channel for transition effect

**Returns:**
- `Dict[str, Any]` - Transition info dictionary matching get_timeline_items() format

## Frame Range Operations

### set_frame_range(start_frame, end_frame)
Set the timeline frame range (start and end frames)

**Parameters:**
- `start_frame` (int) - Starting frame number for the timeline
- `end_frame` (int) - Ending frame number for the timeline

**Returns:**
- `Dict[str, Any]` - Dictionary with success, message, frame_start, frame_end

### get_frame_range()
Get the current timeline frame range and playhead position

**Returns:**
- `Dict[str, Any]` - Dictionary with frame_start, frame_end, frame_current, total_frames

### set_current_frame(frame)
Set the current playhead position on the timeline

**Parameters:**
- `frame` (int) - Frame number to set as current position

**Returns:**
- `Dict[str, Any]` - Dictionary with success, message, frame_current, in_range

### get_current_frame()
Get the current playhead position on the timeline

**Returns:**
- `int` - Current frame number

### modify_image(sequence_name, trim_start=None, trim_end=None, scale=None, position=None, rotation=None)
Modify image sequences on the timeline (trim, zoom, reposition)

**Parameters:**
- `sequence_name` (str) - Name of the sequence to modify
- `trim_start` (Optional[int]) - New start frame for trimming
- `trim_end` (Optional[int]) - New end frame for trimming
- `scale` (Optional[Union[float, Tuple[float, float]]]) - Scale factor - float for uniform or (x, y) tuple for non-uniform
- `position` (Optional[Tuple[float, float]]) - (x, y) offset in pixels from center
- `rotation` (Optional[float]) - Rotation angle in radians

**Returns:**
- `Dict[str, Any]` - Updated sequence info dictionary matching get_timeline_items() format

**Note:** Only works with IMAGE sequence types. Images can be extended beyond their original duration.

### modify_video(sequence_name, trim_start=None, trim_end=None, scale=None, position=None, rotation=None, speed=None)
Modify video sequences on the timeline (trim, zoom, reposition, speed)

**Parameters:**
- `sequence_name` (str) - Name of the sequence to modify
- `trim_start` (Optional[int]) - New start frame for trimming
- `trim_end` (Optional[int]) - New end frame for trimming
- `scale` (Optional[Union[float, Tuple[float, float]]]) - Scale factor - float for uniform or (x, y) tuple for non-uniform
- `position` (Optional[Tuple[float, float]]) - (x, y) offset in pixels from center
- `rotation` (Optional[float]) - Rotation angle in radians
- `speed` (Optional[float]) - Playback speed multiplier (0.1-10.0, where 1.0 = normal speed)

**Returns:**
- `Dict[str, Any]` - Updated sequence info dictionary matching get_timeline_items() format

**Note:** Only works with MOVIE sequence types. Videos can only be trimmed, not extended beyond their original duration. Speed control affects playback rate and effective duration.

### modify_audio(sequence_name, trim_start=None, trim_end=None, volume=None, pan=None)
Modify audio sequences on the timeline (trim, volume, pan)

**Parameters:**
- `sequence_name` (str) - Name of the sequence to modify
- `trim_start` (Optional[int]) - New start frame for trimming
- `trim_end` (Optional[int]) - New end frame for trimming
- `volume` (Optional[float]) - Volume level (0.0-100.0, where 100.0 is full volume)
- `pan` (Optional[float]) - Stereo panning (-inf to +inf, 0.0 is center, only for mono sources)

**Returns:**
- `Dict[str, Any]` - Updated sequence info dictionary matching get_timeline_items() format

**Note:** Only works with SOUND sequence types. Other types (IMAGE, MOVIE, TEXT, COLOR, effects) will raise an error.

### detach_audio_from_video(video_sequence_name, audio_channel, audio_name=None)
Detach audio from a video sequence and create a separate audio sequence

**Parameters:**
- `video_sequence_name` (str) - Name of the video sequence to detach audio from
- `audio_channel` (int) - Channel number to place the detached audio sequence
- `audio_name` (Optional[str]) - Name for the audio sequence (auto-generated if None)

**Returns:**
- `Dict[str, Any]` - Dictionary containing:
  - `success` (bool) - Whether the operation succeeded
  - `message` (str) - Success/error message  
  - `video_sequence` (Dict[str, Any]) - Original video sequence info
  - `audio_sequence` (Optional[Dict[str, Any]]) - Created audio sequence info (None if no audio)

**Note:** Only works with MOVIE sequence types. Creates a new SOUND sequence from the video's audio track. If the video has no audio, returns success=False with appropriate message.

## Timeline operations

### export_video(output_path, frame_start=None, frame_end=None, resolution=(1920, 1080), fps=24, video_format='MPEG4', codec='H264', quality='HIGH')
Export timeline sequences to a video file

**Parameters:**
- `output_path` (str) - Path for the output video file
- `frame_start` (Optional[int]) - Starting frame (auto-detected if None)
- `frame_end` (Optional[int]) - Ending frame (auto-detected if None)
- `resolution` (Tuple[int, int]) - Video resolution as (width, height) tuple
- `fps` (int) - Frames per second
- `video_format` (str) - Video container format ('MPEG4', 'AVI', 'QUICKTIME', 'WEBM')
- `codec` (str) - Video codec ('H264', 'XVID', 'THEORA', 'VP9')
- `quality` (str) - Quality preset ('LOW', 'MEDIUM', 'HIGH', 'LOSSLESS')

**Returns:**
- `Dict[str, Any]` - Export info with output_path, frame_start, frame_end, duration, resolution, fps, file_size, success

### capture_preview_frame(output_path, frame=None, resolution=(1920, 1080), format='PNG', quality=90)
Capture a screenshot of the current preview in the video editor

**Parameters:**
- `output_path` (str) - Path for the output image file
- `frame` (Optional[int]) - Frame number to capture (uses current frame if None)
- `resolution` (Tuple[int, int]) - Image resolution as (width, height) tuple
- `format` (str) - Image format ('PNG', 'JPEG', 'TIFF', 'BMP', 'TARGA')
- `quality` (int) - Image quality for JPEG format (1-100)

**Returns:**
- `Dict[str, Any]` - Capture info with output_path, frame, resolution, format, file_size, success

**Supported formats:** PNG (default), JPEG, TIFF, BMP, TARGA

## Video Resize Detection

To detect if a video has been resized after loading in Blender:

```python
# Check all sequences for resizing
sequences = get_timeline_items()
resized_sequences = [seq for seq in sequences if seq['is_resized']]

# Get detailed info for a specific sequence
resize_info = get_sequence_resize_info('my_video')
if resize_info['is_resized']:
    print(f"Video scaled from {resize_info['original_resolution']} to {resize_info['effective_resolution']}")
```