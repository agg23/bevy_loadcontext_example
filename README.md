# LoadContext bug

It seems Bevy's `LoadContext` sometimes drops handles to asset resources, causing assets to silently have missing parts.

### Expected:
![Expected output](assets/correct.png)

### Actual (this happens 75% of the time)
![Actual](assets/possible.png)

### Inspector

The meshes are still there, but the material handle is invalid

![Inspector](assets/inspector.png)